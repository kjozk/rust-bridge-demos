# GitHub Issue 用 Agents 運用設計 README

本フォルダーは、GitHub Issue にアサインして利用する AI Agent 群の定義および運用方針をまとめたものです。

目的：

* Issue 処理を用途別 Agent に分離する
* コストと精度を人間がアサイン時に制御する
* Enterprise 管理権限なしで実運用可能にする

---

## 運用方針（重要）

* 管理者側でモデル固定は行わない
* Agent ごとに用途・コスト帯・役割を明確化
* Issue アサイン時に人間が Agent を選択

推奨運用フロー：

1. 新規 Issue 作成
2. まず「軽量トリアージ Agent」をアサイン
3. 内容に応じて専門 Agent に切替

   * 難度高 → Claude Agent
   * 整理目的 → 軽量 Agent

---

## Agent 定義一覧

以下は `.github/agents/*.yaml` に配置する想定のサンプル定義です。

---

## ① 軽量トリアージ Agent（通常トリアージ）

### 用途

* 新規 Issue の初期整理
* 種別・重大度・不足情報の抽出
* ラベル・担当割当の補助

### 想定モデル

* 軽量・低コストモデル（GPT-4o-mini / Claude Haiku 等）

### 定義例

```yaml
name: 軽量トリアージ Agent
description: |
  【用途】
  新規 Issue の初期トリアージ専用 Agent。
  Bug / Feature / Question の分類、重大度推定、不足情報の抽出を行う。

  【主な振る舞い】
  - Issue 本文とテンプレを解析
  - 種別・重大度・再現性を推定
  - 不足情報を指摘
  - ラベル付与や担当割当の提案コメントを投稿

  【利用タイミング】
  - すべての新規 Issue に最初にアサインする

  【コスト方針】
  - 常時使用するため低コストモデルを使用

model: gpt-4o-mini
capabilities:
  - issue_triage
  - labeling
  - summarization
```

---

## ② 要約・整理 Agent（Issue 整形・状況要約）

### 用途

* 長文化した Issue の状況整理
* 議論・決定事項・未決事項の要約
* 引き継ぎ用ブリーフ作成

### 定義例

```yaml
name: 要約・整理 Agent
description: |
  【用途】
  コメントが多くなった Issue の状況整理専用 Agent。
  議論の経緯、決定事項、未解決事項、次アクションを要約する。

  【主な振る舞い】
  - 全コメントを時系列で解析
  - 現在の論点とステータスを整理
  - 冒頭またはコメントに要約を投稿

  【利用タイミング】
  - コメントが増えた Issue
  - 途中参加者への共有

  【コスト方針】
  - 精度より速度重視、中〜低コストモデル

model: gpt-4o-mini
capabilities:
  - summarization
  - issue_analysis
```

---

## ③ バグ解析 Agent（原因仮説・調査支援 / Claude 推奨）

### 用途

* スタックトレース・ログ解析
* 原因候補の提示
* 調査方針・修正方針の提案

### 定義例

```yaml
name: バグ解析 Agent（高精度）
description: |
  【用途】
  再現条件・ログ・スタックトレースを解析し、原因仮説と調査方針を提示する高精度 Agent。

  【主な振る舞い】
  - 例外型・発生箇所・呼び出し経路を解析
  - 原因候補を複数提示（確度付き）
  - 追加ログ位置やテスト方針を提案

  【利用タイミング】
  - 原因不明の不具合
  - 本番影響あり

  【コスト方針】
  - 高コストだが最も費用対効果が高い用途

model: claude-3.5-sonnet
capabilities:
  - bug_analysis
  - log_analysis
  - root_cause_hypothesis
```

---

## ④ 設計レビュー Agent（アーキテクチャ・仕様レビュー / Claude 推奨）

### 用途

* API 設計レビュー
* 破壊的変更チェック
* 拡張性・責務分離の確認

### 定義例

```yaml
name: 設計レビュー Agent（高精度）
description: |
  【用途】
  設計 Issue を対象に、アーキテクチャ・責務分離・拡張性・互換性をレビューする Agent。

  【主な振る舞い】
  - 設計目的と制約を整理
  - 懸念点・破壊的変更・保守性を指摘
  - 改善案を提示

  【利用タイミング】
  - 新 API 設計
  - 大規模リファクタ
  - 長期保守コード

  【コスト方針】
  - 高コストだが設計ミス防止の効果が大きい

model: claude-3.5-sonnet
capabilities:
  - design_review
  - architecture_analysis
```

---

## ⑤ 仕様抽出・ドキュメント化 Agent

### 用途

* Issue / PR から仕様を抽出
* 後追いドキュメント作成
* API 仕様ドラフト生成

### 定義例

```yaml
name: 仕様抽出・ドキュメント化 Agent
description: |
  【用途】
  Issue・コメント・PR から仕様を抽出し、ドキュメント形式に整理する Agent。

  【主な振る舞い】
  - 入出力仕様・振る舞いを構造化
  - 仕様ドラフトを生成

  【利用タイミング】
  - 実装先行案件
  - ドキュメント未整備案件

model: gpt-4o
capabilities:
  - spec_extraction
  - documentation
```

---

## ⑥ コードレビュー Agent

### 用途

* PR 差分レビュー
* バグ・品質・パフォーマンス指摘
* 初学者レビュー補助

### 定義例

```yaml
name: コードレビュー Agent
description: |
  【用途】
  PR 差分を解析し、品質・安全性・可読性・性能の観点でレビューする Agent。

  【主な振る舞い】
  - 危険な実装・例外処理・境界条件を指摘
  - 改善案を提示

  【利用タイミング】
  - 大型 PR
  - セキュリティ影響あり

model: gpt-4o
capabilities:
  - code_review
  - static_analysis
```

---

## 推奨モデル割当ポリシー

| 用途      | 推奨モデル     | 理由          |
| ------- | --------- | ----------- |
| トリアージ   | 軽量        | 常時使用・コスト最小化 |
| 要約整理    | 軽量        | 精度要求低め      |
| バグ解析    | Claude    | 推論力・因果分析が重要 |
| 設計レビュー  | Claude    | 設計ミスの損失が大きい |
| 仕様抽出    | 中位〜Claude | 正確性重視       |
| コードレビュー | 中位〜Claude | 品質影響あり      |

---

## 最重要運用原則

* Agent = 役割
* Model = コスト・精度の実装
* 選択権は常に人間が持つ

この方式により：

* 管理者権限不要
* モデル誤爆防止
* コスト最適化
* 高難度のみ Claude 利用

が同時に実現されます。

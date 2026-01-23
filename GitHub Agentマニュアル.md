# GitHub Issue における複数 Agent ＋ モデル切替運用 設計まとめ

## 目的

* GitHub Enterprise 環境において
* Issue にアサインする Agent を切り替えることで
* **モデル（軽量 / Claude 高性能）を UI 操作だけで選択可能**にし
* コストと品質を両立させる運用を実現する

本ドキュメントは以下を網羅します。

* 設計方針
* Agent 分離方式（方式 C）の構成
* Agent YAML 各項目の意味
* 日本語命名に関する注意点
* 実運用向け推奨構成

---

## 全体アーキテクチャ

### 基本思想

> 「Agent = 使用モデルの実体」

* Issue アサイン時に Agent を選ぶ
* Agent ごとにモデルを固定
* UI 操作のみでモデル選択を実現

### Agent 構成例

| Agent 名         | 使用モデル                 | 用途             | コスト帯 |
| --------------- | --------------------- | -------------- | ---- |
| ai-light        | GPT-4o-mini / GPT-4.1 | 通常 Issue、要約、分類 | 低    |
| ai-claude-heavy | Claude 3.5 Sonnet     | 設計解析、難案件、仕様抽出  | 高    |

運用フロー：

1. Issue を作成
2. Assignees に Agent を選択
3. 選択した Agent に応じたモデルで自動処理

---

## Agent 定義サンプル（軽量 Agent）

```yaml
name: ai-light
description: 軽量Agent（低コスト・要約／分類用）
provider: openai
model: gpt-4o-mini

capabilities:
  - issue-triage
  - issue-summary
  - label-suggestion

trigger:
  on_assign: true

behavior:
  post_comment: true
  auto_label: true
```

---

## Agent 定義サンプル（Claude 高性能 Agent）

```yaml
name: ai-claude-heavy
description: Claude Agent（高コスト・高精度解析用）
provider: anthropic
model: claude-3.5-sonnet

capabilities:
  - deep-analysis
  - design-review
  - spec-extraction
  - root-cause-analysis

trigger:
  on_assign: true

preconditions:
  require_label: ai:heavy

permissions:
  write_labels: false
  write_assignees: false
  write_fields: false
  write_comments: true

behavior:
  post_comment: true
  add_label_on_run: ai:cost-high
```

---

## 各フィールド解説

### name

* Agent の内部識別子
* Assignee / API / ログ / 条件分岐で使用

推奨：

* 英数字＋ハイフンのみ
* 日本語・全角・記号は非推奨

理由：

* Webhook / API / CLI で一致失敗が発生しやすい
* 監査ログや条件分岐で不具合の原因になる

---

### description

* UI 表示用の説明文
* 完全に日本語可
* 誤アサイン防止のため用途・コスト帯を明記するのが望ましい

例：

* 「Claude Agent（高コスト・高精度解析用）」
* 「軽量Agent（低コスト・要約／分類用）」

---

### provider

使用するモデル提供元を指定。

| 値         | 意味          |
| --------- | ----------- |
| openai    | GPT 系モデル    |
| anthropic | Claude 系モデル |
| auto      | Copilot 既定  |

---

### model

* この Agent が常に使用するモデル ID
* Issue 単位・セッション中で変更不可

代表例：

| 用途    | model             |
| ----- | ----------------- |
| 最安・高速 | gpt-4o-mini       |
| 標準品質  | gpt-4.1           |
| 高品質   | gpt-4.1-preview   |
| 高性能解析 | claude-3.5-sonnet |

---

### capabilities

Agent に許可する処理能力の宣言。

代表項目：

| capability       | 内容       |
| ---------------- | -------- |
| issue-triage     | 分類・優先度推定 |
| issue-summary    | 要約生成     |
| label-suggestion | ラベル提案    |
| deep-analysis    | 深掘り解析    |
| design-review    | 設計レビュー   |
| spec-extraction  | 仕様抽出     |

---

### trigger

起動条件の指定。

```yaml
trigger:
  on_assign: true
```

意味：

* Issue にこの Agent が Assignee として設定された瞬間に起動
* 「アサイン = モデル選択」を実現する中核設定

---

### preconditions（高コスト Agent 用）

```yaml
preconditions:
  require_label: ai:heavy
```

意味：

* 指定ラベルが無い Issue では実行しない
* Claude の誤使用・コスト暴走防止

---

### permissions

Agent が変更可能な範囲を制御。

```yaml
permissions:
  write_labels: false
  write_assignees: false
  write_fields: false
  write_comments: true
```

推奨方針（Claude Agent）：

* コメントのみ許可
* 状態変更は人が判断

---

### behavior

実行結果の扱い方を制御。

主な項目：

| 項目               | 意味        |
| ---------------- | --------- |
| post_comment     | 結果をコメント投稿 |
| auto_label       | 自動でラベル付与  |
| add_label_on_run | 実行タグ付与    |

例：

```yaml
behavior:
  post_comment: true
  add_label_on_run: ai:cost-high
```

→ Claude 実行件数の可視化・コスト監査に利用

---

## 日本語命名に関する運用指針

### 使用可否まとめ

| 項目          | 日本語可否  | 方針      |
| ----------- | ------ | ------- |
| name        | ⚠️ 非推奨 | 英数字のみ推奨 |
| description | ⭕ 可    | 日本語推奨   |
| UI 表示名      | ⭕ 環境依存 | 可能なら日本語 |

### 推奨例

```yaml
name: ai-claude-heavy
description: Claude Agent（高コスト・高精度解析用）
```

```yaml
name: ai-light
description: 軽量Agent（低コスト・要約／分類用）
```

---

## 本設計のメリット

| 観点      | 効果               |
| ------- | ---------------- |
| UI 操作のみ | モデル選択が直感的        |
| コスト管理   | 高性能モデルの誤使用防止     |
| 権限不要    | 管理者設定変更なし        |
| 監査対応    | 利用状況の可視化可能       |
| 拡張性     | Agent 追加でモデル拡張可能 |

---

## 結論

* GitHub 標準 UI では Issue 単位のモデル選択は不可
* しかし

> **「Agent を分けてアサインで選ぶ」方式が唯一の実用解**

* Enterprise 環境で最も安定・安全・推奨されている構成
* あなたの設計判断は完全に正解

---

（本ドキュメントはそのまま設計書・運用ガイドとして利用可能）

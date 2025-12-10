# 第2回：変数と型 - 練習問題

> **ブログ記事：** （公開後にリンクを追加）  
> **学習ガイド：** [docs/02_variables.md](../../docs/02_variables.md)

このディレクトリには、第2回「変数と型」の練習問題が含まれています。

---

## 📝 練習問題一覧

### 基本編

1. **ex01_basic** - 変数の基本
   - 不変変数と可変変数
   - 型の明示

2. **ex02_shadowing** - シャドーイング
   - 変数の再宣言
   - 型の変更

3. **ex03_types** - データ型
   - 整数型
   - 浮動小数点数型
   - 真偽値型
   - 文字型

4. **ex04_compound** - 複合型
   - タプル
   - 配列

### 応用編

5. **ex05_conversion** - 型変換
   - asキーワード
   - parse()メソッド

6. **ex06_constants** - 定数
   - constの使い方

### チャレンジ

7. **challenge01_bmi** - BMI計算機
8. **challenge02_temperature** - 温度変換
9. **challenge03_student** - 学生情報管理
10. **challenge04_scores** - テスト結果集計

---

## 🚀 実行方法

### 練習問題一覧を表示

```bash
cd exercises/02_variables
cargo run --bin variables_exercises
```

または（Cargo.tomlでdefault-runを設定済みなら）
```bash
cargo run
```

### 個別の練習問題を実行

```bash
# 練習問題1を実行
cargo run --bin ex01_basic

# 練習問題2を実行
cargo run --bin ex02_shadowing

# チャレンジ1を実行
cargo run --bin challenge01_bmi
```

**便利なコマンド：**

```bash
# すべての練習問題をビルド（エラーチェック）
cargo build

# 特定の練習問題だけビルド
cargo build --bin ex01_basic
```

---

## ✏️ 練習の進め方

1. **学習ガイドを読む**
   - [docs/02_variables.md](../../docs/02_variables.md)

2. **コードを読む**
   - `src/bin/` 内の各ファイル

3. **コードを編集**
   - コメントの指示に従ってコードを完成させる

4. **実行して確認**
   - `cargo run --bin ファイル名`

5. **次の問題へ**
   - できたら次の練習問題に進む

---

## 💡 ヒント

### エラーが出たら

1. エラーメッセージをよく読む
2. 該当する行を確認
3. 学習ガイドを参照
4. 解答例を見る（最終手段）

### 詰まったら

- 学習ガイドの該当セクションを読み直す
- Rustのエラーメッセージには解決方法が書いてある
- 公式ドキュメントを参照

---

## 📊 進捗チェックリスト

- [ ] ex01_basic - 変数の基本
- [ ] ex02_shadowing - シャドーイング
- [ ] ex03_types - データ型
- [ ] ex04_compound - 複合型
- [ ] ex05_conversion - 型変換
- [ ] ex06_constants - 定数
- [ ] challenge01_bmi - BMI計算機
- [ ] challenge02_temperature - 温度変換
- [ ] challenge03_student - 学生情報
- [ ] challenge04_scores - テスト結果

---

## 次のステップ

すべての練習問題を完了したら、第3回「関数」に進みましょう！

**👉 第3回の学習ガイド**（準備中）

---

**Happy Coding!** 🦀
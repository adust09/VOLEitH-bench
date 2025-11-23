VOLE-in-the-Head（VOLEitH）は、耐量子署名スキームであるFAEST や、zkPassプロトコル のようなアプリケーションで使用される、効率的で公開検証可能なゼロ知識証明（ZKP）を構築するための新しいアプローチです。これは、**MPC-in-the-Head (MPCitH)** パラダイムの改良版として位置づけられています。

VOLEitHの仕組みは、主に**VOLE（Vector Oblivious Linear Evaluation）相関**の概念と、それを非対話的かつ公開検証可能な形式に変換する**コンパイラ（compiler）**技術に基づいています。

以下に、VOLEitHの仕組みを数学的・学術的な観点から詳述します。

### 1. VOLE相関（VOLE Correlation）の基礎

VOLEitHは、線形準同型コミットメントスキームの一種として、有限体 $\text{F}_{2^k}$ 上のVOLE相関を利用します。

長さ $\ell$ のVOLE相関は、以下の**VOLE関係式**を満たす要素によって定義されます：

$$\mathbf{q}_i = \mathbf{u}_i \cdot \mathbf{\Delta} + \mathbf{v}_i \quad \text{for } i = 0, \ldots, \ell-1 \quad (1)$$

*   **$\mathbf{\Delta}$（グローバルキー）**: 検証者 $V$ に与えられるランダムなキー ($\mathbf{\Delta} \in \text{F}_{2^k}$)。
*   **$\mathbf{u}_i$（ランダムビット）**: 証明者 $P$ のみが知るランダムなビットの集合 ($\mathbf{u}_i \in \text{F}_{2}$)。
*   **$\mathbf{v}_i$（VOLEタグ）**: $P$ のみが知るランダムなマスク値 ($\mathbf{v}_i \in \text{F}_{2^k}$)。
*   **$\mathbf{q}_i$（VOLEキー）**: $V$ に与えられる値 ($\mathbf{q}_i \in \text{F}_{2^k}$)。

このVOLE相関は、**線形準同型コミットメントスキーム**として機能します。
1.  **秘匿性 (Hiding)**：ランダムなタグ $\mathbf{v}_i$ が検証者 $V$ の持つ $\mathbf{q}_i$ の中で $\mathbf{u}_i$ をマスクすることで、秘匿性が保証されます。
2.  **拘束性 (Binding)**：$\mathbf{u}_i$ と異なる値 $\mathbf{u}'_i$ でコミットメントを開示しようとすると、証明者 $P$ はグローバルキー $\mathbf{\Delta}$ を推測する必要があり、成功確率は $2^{-k}$ に制限されます。これは、コミットメントの拘束性を保証します。
3.  **準同型性 (Homomorphism)**：同じ $\mathbf{\Delta}$ の下で生成された情報理論的メッセージ認証コード（MAC）は、加法的に準同型です。

### 2. VOLEitHの非対話化プロセス

本来、VOLE相関はセキュアな二者間プロトコル（secure two-party protocol）によって生成されますが、VOLEitH技術は、これを**非対話的**かつ**公開検証可能**なZKP（NIZK）にするために採用されます。

VOLEitHの目標は、$P$ がランダムベクトル $\mathbf{u} \in \{0, 1\}^{\ell}$ と $\mathbf{v} \in \text{F}_{2^\lambda}^{\ell}$ にコミットし、$V$ がランダムなチャレンジ $\mathbf{\Delta} \in \text{F}_{2^\lambda}$ を知った後に、$P$ が $\mathbf{q} = \mathbf{u} \cdot \mathbf{\Delta} - \mathbf{v}$ を満たす $\mathbf{q}$ を開示できるようにすることです。

この非対話化は、主に以下の構成要素によって実現されます。

#### 2.1. All-but-One Vector Commitment (VC)
VOLEitHの主要な構成要素は、**疑似ランダムな $N$ 個のシードベクトルにコミットする**ための技術です。これは、**GGM (Goldreich–Goldwasser–Micali) 構造**に基づいた**穿孔可能擬似ランダム関数（puncturable PRF）**を用いて構築される**All-but-One Vector Commitment**としてモデル化されます。

1.  **コミットメント**: $P$ は、単一のハッシュ値 $h$ を送ることで $N$ 個のシード $\mathbf{sd}_0, \ldots, \mathbf{sd}_{N-1}$ にコミットします。コミットメント $h$ は、GGMツリーの葉から導出された補助コミットメント $\mathbf{com}_j$ をハッシュ関数 $H_1$ でハッシュすることで計算されます。
2.  **開示**: $P$ は、ランダムに選ばれたインデックス $j^*$ を除く $N-1$ 個のシードを開示できます。この開示は、GGMツリーの根から葉 $j^*$ への経路上の**すべての兄弟ノード**を $V$ に送るだけで完了し、通信量は $O(\log N)$ です。

このVCスキームでは、検証者 $V$ は部分的な開示（$\mathbf{pdecom}$）を受け取り、それを $\mathbf{VC.Reconstruct}$ アルゴリズムにかけて、すべてのシード（開示されなかった $j^*$ を除く）と、コミットメント $h$ を再構築し、元のコミットメント $h_{\mathbf{com}}$ と比較して検証します。

#### 2.2. シードのVOLE相関への変換
VCによってコミットされた $N$ 個の疑似ランダムなシード $\mathbf{sd}_0, \ldots, \mathbf{sd}_{N-1}$ は、VOLE相関に変換されます。

1.  **シードの展開**: $P$ は、各シード $\mathbf{sd}_i$ をPRGを用いて長さ $\ell$ の文字列 $\mathbf{r}_i \in \{0, 1\}^{\ell}$ に展開します。
2.  **VOLE値の計算**: $P$ は、展開された文字列 $\mathbf{r}_i$ を用いて、有限体 $\text{F}_{2^k}$ において以下の $\mathbf{u}$ と $\mathbf{v}$ を計算します:
    $$\mathbf{u} = \sum_{i=0}^{N-1} \mathbf{r}_i, \quad \mathbf{v} = \sum_{i=0}^{N-1} i \cdot \mathbf{r}_i$$
    ここで、 $i$ は $\text{F}_{2^k}$ の要素としてエンコードされます。
3.  **$V$ によるVOLEキーの計算**: $V$ は、開示されなかったインデックス $j^*$（グローバルキー $\mathbf{\Delta}$ に対応）を使用して、以下の $\mathbf{q}$ を計算できます：
    $$\mathbf{q} = \sum_{i=0}^{N-1} (j^* - i) \cdot \mathbf{r}_i = j^* \cdot \mathbf{u} - \mathbf{v}$$
    これにより、望ましいVOLE相関 $\mathbf{q} = \mathbf{u} \cdot \mathbf{\Delta} - \mathbf{v}$（ただし $\mathbf{\Delta} = j^*$）が得られます。

#### 2.3. 並列インスタンス化と整合性チェック
単一のVOLEインスタンスでセキュリティパラメータ $\lambda$（例: 128ビット）を満たすために $\text{F}_{2^\lambda}$ を直接使用することは、計算量的に非現実的です。

そこで、VOLEitHは、より小さいフィールド $\text{F}_{2^{k_0}}$ および $\text{F}_{2^{k_1}}$ 上で複数の**並列インスタンス ($\tau$ 個)** を実行し、その出力を連結することで $\text{F}_{2^\lambda}$ 上のVOLE相関を構築します。

VOLECommitアルゴリズムは、$\tau$ 個のVC.Commitを実行し、それぞれをConvertToVOLEを用いてVOLE相関に圧縮します。最終的に、$\mathbf{u}$ ベクトルを統一するための**補正値 $\mathbf{c}_i$**（Correction values）が計算され、$V$ に送られます。

また、証明者 $P$ が補正値 $\mathbf{c}_i$ を不正に生成していないことを保証するために、**VOLE整合性チェック（VOLE Consistency Check）**が実行されます。ここでは、$V$ がランダムで線形なユニバーサルハッシュ関数を $\mathbf{u}$ と $\mathbf{V}$ に適用するように $P$ に要求します。

### 3. ZKPへの統合と応用

VOLE-in-the-Head（VOLEitH）は、Fiat-Shamir変換 を用いて、VOLEベースの対話型ゼロ知識証明（ZKP）プロトコル（QuickSilverなど）を非対話型で**公開検証可能**な形式に変換するためのフレームワークです。

VOLEitHプロトコル全体は、主に**VOLE相関の非対話的な生成**、**QuickSilver検証ロジックの実行**、および**Fiat-Shamir変換による検証**の3つの主要な段階で構成されており、証明者 ($P$) と検証者 ($V$) の役割は以下の通りです。

---

## VOLE-in-the-Head 証明プロトコル全体解説

VOLEitHの基本的な目標は、$P$がランダムなベクトル $\mathbf{u} \in \{0, 1\}^{\ell}$ と $\mathbf{v} \in \text{F}_{2^\lambda}^{\ell}$ にコミットし、$V$がランダムなチャレンジ $\mathbf{\Delta} \in \text{F}_{2^\lambda}$ を知った後、$P$がVOLE関係式 $\mathbf{q} = \mathbf{u} \cdot \mathbf{\Delta} - \mathbf{v}$ を満たすベクトル $\mathbf{q}$ を開示できるようにすることです。

### ステージ 1: VOLE相関の生成とコミットメント（$P$の事前準備）

この段階では、$P$は自身の秘密となる乱数（$\mathbf{u}$と$\mathbf{v}$）を生成し、コミットメントを行い、$V$は最終的な検証に必要なハッシュ値と補正値を受け取ります。

#### 1.1. オール・バット・ワン・ベクトルコミットメント（VC）の利用

VOLEitHは、GGM（Goldreich–Goldwasser–Micali）構造に基づく**All-but-One Vector Commitment (VC)** を主要な構成要素として利用します。これは、単一のハッシュ値 $\mathbf{h}_{\text{com}}$ を送信することで、多数の疑似乱数シードにコミットする手法です。

*   **$P$の行動**:
    1.  $P$はランダムなシード $r$ を使用して、長さ倍の擬似乱数生成器（PRG）に基づく**完全二分木（GGMツリー）**を構築します。
    2.  $P$は、ツリーの葉から得られた $N$ 個のキー $\mathbf{k}_i$ を、ハッシュ関数 $H_0$ にかけて、 $N$ 個のシード $\mathbf{sd}_i$ と補助コミットメント $\mathbf{com}_i$ を生成します。
    3.  $P$は、すべての補助コミットメント $\mathbf{com}_i$ を別のハッシュ関数 $H_1$ に入力し、**最終的なコミットメント $\mathbf{h}_{\text{com}}$** を計算します。
    4.  $P$は、各シード $\mathbf{sd}_i$ をPRGを用いて文字列 $\mathbf{r}_i$ に展開し、VOLEの秘密$\mathbf{u} = \sum \mathbf{r}_i$とVOLEタグ$\mathbf{v} = \sum i \cdot \mathbf{r}_i$を計算します。
    5.  $P$は、複数のVOLEインスタンス間での$\mathbf{u}$の一貫性を保証するために、**補正値 $\mathbf{c}_i$**を計算します。

*   **$P$から$V$への送信**: $P$は $\mathbf{h}_{\text{com}}$ と $\tau-1$ 個の補正値 $\mathbf{c}_i$ を $V$ に送ります。

### ステージ 2: QuickSilver ZK証明の実行とコミットメント（$P$の計算）

$P$は、秘密の証拠（witness）$\mathbf{w}$が公開された関係式を満たしていることを証明します。VOLEitHは、QuickSilverプロトコルに基づいて、線形準同型性を利用した効率的なチェックを実行します。

#### 2.1. 証拠のマスキングとコミット

$P$は、証明対象の計算（例えば、ある一方向関数 $F(\mathbf{w})=\mathbf{y}$）の実行中に記録された**拡張された証拠 $\mathbf{w}$** を用います。

*   **$P$の行動**:
    1.  $P$は、VOLE相関の秘密 $\mathbf{u}$ を使用して証拠 $\mathbf{w}$ を排他的論理和（XOR）でマスクし、**マスクされた証拠 $\mathbf{d}$** を計算します: $\mathbf{d} = \mathbf{w} \oplus \mathbf{u}[0..\ell)$。
    2.  $P$は、この$\mathbf{d}$を$V$に送信します。

#### 2.2. 乗算ゲートの検証（QuickSilverコア）

QuickSilverは、線形演算（加算、定数乗算）はVOLE MACsの準同型性により通信なしで処理し、非線形演算（乗算）のみをチェックします。

*   **$P$の行動**:
    1.  $P$は、回路内の各乗算ゲート $(\alpha, \beta, \gamma)_i$ について、**非線形項の検証に必要な係数** $\mathbf{a}_{0,i}$ と $\mathbf{a}_{1,i}$ を計算します。これらの係数は、証拠 $\mathbf{w}$ とVOLEタグ $\mathbf{v}$ にのみ依存します。
    2.  $P$は、これらの $t$ 組の係数を線形なユニバーサルハッシュ関数（ZKHash）やランダムなチャレンジ $\mathbf{\chi}$ を用いて集約し、単一の組 $(\mathbf{a}_0, \mathbf{a}_1)$ を生成します。
    3.  $P$は、ゼロ知識性（ZK）を保証するために、補助的なランダムVOLE MAC ($\mathbf{a}^*_0, \mathbf{a}^*_1$) を用いて集約された係数 $(\mathbf{a}_0, \mathbf{a}_1)$ をマスクします。
    4.  $P$は、このマスクされた結果を、Fiat-Shamir変換によって生成されたチャレンジ $\mathbf{chall}_2$ を鍵とする**$\text{ZKHash}$**を用いてハッシュし、**コンパクトな証明コンポーネント $(\tilde{\mathbf{a}}, \tilde{\mathbf{b}})$** を導出します。

### ステージ 3: Fiat-Shamir変換と最終検証（$V$の検証）

この最終段階では、Fiat-Shamir変換によりチャレンジが確定し、コミットメントの開放と証明の検証が行われます。

#### 3.1. チャレンジの生成とコミットメントの開放

*   **$P$の行動**: $P$は、$\mathbf{chall}_2$と証明 $(\tilde{\mathbf{a}}, \tilde{\mathbf{b}})$ を含むトランスクリプト全体をハッシュし、**最終的なチャレンジ $\mathbf{chall}_3$** を生成します。この $\mathbf{chall}_3$ は、VOLE相関のグローバルキー $\mathbf{\Delta}$ となり、また、VCの**未開封インデックス**を決定します。
*   **$P$から$V$への送信**: $P$は、$\mathbf{chall}_3$に応じて各VCインスタンスの**部分的な非コミットメント情報 $\mathbf{pdecom}_i$** （未開封インデックスへの経路上の兄弟ノードのキー）を計算し、署名 $\mathbf{\sigma}$ の一部として $V$ に送信します。

#### 3.2. $V$によるVOLEキーの再構築と整合性チェック

$V$は受け取った署名情報を用いて、VOLE相関の整合性を検証します。

*   **$V$の行動**:
    1.  $V$は $\mathbf{chall}_3$ からグローバルキー $\mathbf{\Delta}$ を再構築します。
    2.  $V$は $\mathbf{pdecom}_i$ を使用して**$\mathbf{VC.Reconstruct}$**を実行し、コミットメント $\mathbf{h}_{\text{com}}$ が正しく計算されたことを検証し、同時にすべての$\mathbf{r}_i$から**VOLEキー行列 $\mathbf{Q}$**を再構築します。
    3.  $V$は、 ステージ1で$P$から受け取った**補正値 $\mathbf{c}_i$** を $\mathbf{Q}$ に適用し、すべての$\tau$インスタンスが同じ秘密$\mathbf{u}$に紐付いていることを保証します。
    4.  $V$は、**マスクされた証拠 $\mathbf{d}$**と$\mathbf{\Delta}$を用いて、$\mathbf{Q}$の最初の部分（証拠に対応する部分）を修正し、$\mathbf{Q}$が実際に**証拠 $\mathbf{w}$ のVOLEキー**となっていることを確認します。

#### 3.3. 最終的なQuickSilver検証

*   **$V$の行動**:
    1.  $V$は、修正されたVOLEキー $\mathbf{Q}$ と公開された入力を用いて、QuickSilver検証ロジック（例えば、$\mathbf{AESVerify}$）を実行し、**乗算ゲート検証**に必要なMACキー $\mathbf{b}$ を再計算します。
    2.  $V$は、この再計算されたMACキー $\mathbf{b}$ と、ZKのためのマスキングキー $\mathbf{q}^*$ を結合し、$\mathbf{chall}_2$ を使用した$\text{ZKHash}$によって**期待される最終MACキー $\tilde{\mathbf{q}}$** を計算します。
    3.  $V$は、 $P$から受け取った証明コンポーネント $\tilde{\mathbf{a}}, \tilde{\mathbf{b}}$ と $\mathbf{\Delta}$ を用いて、最終的な検証式が成立するかチェックします。

    $$\tilde{\mathbf{q}} - \tilde{\mathbf{a}} \cdot \mathbf{\Delta} \stackrel{?}{=} \tilde{\mathbf{b}}$$

    4.  この式が成立し、かつ$\mathbf{chall}_3$が$V$によって再計算された値と一致する場合、証明は受理されます。

VOLEitHの効率の核心は、このQuickSilver検証プロセスにあります。従来のMPCitHが $N$ 個のMPC参加者のシミュレーションを必要としたのに対し、VOLEitHは$\text{F}_{2^\lambda}$上の線形準同型コミットメントスキームに依存することで、回路の検証手順を大幅に簡素化しています。これにより、線形演算を自然に処理でき、証明生成に必要な計算量とメモリ消費が削減されます。

---
**簡潔なアナロジー**:

VOLEitHによる証明は、**「秘密の設計図を公開鍵でロックされた金庫に収め、その金庫の開錠情報のみを公開する」**行為に似ています。

1.  $P$は、秘密の証拠（$\mathbf{w}$）を、VOLE相関という特殊な**「合鍵情報」** ($\mathbf{u}, \mathbf{v}$) で暗号化し、その合鍵情報に**「開錠不能なコミットメント」** ($\mathbf{h}_{\text{com}}$) を行います。
2.  $P$は、暗号化された証拠が正しいことを、QuickSilverの仕組みを使って証明する中間結果 ($\tilde{\mathbf{a}}, \tilde{\mathbf{b}}$) を作成します。
3.  $V$から要求されるチャレンジ（$\mathbf{\Delta}$）は、ハッシュ値 ($\mathbf{chall}_3$) として生成されます。
4.  $P$は、$\mathbf{chall}_3$によって指定された部分のみを公開します。これは、金庫の鍵自体ではなく、**金庫の仕組みを再構築し、合鍵が正しく作成されたことを検証するために必要な部分的な情報**（$\mathbf{pdecom}_i$）です。
5.  $V$は、公開された断片情報と$\mathbf{\Delta}$を用いて、金庫の合鍵 ($\mathbf{Q}$) と証明結果 ($\tilde{\mathbf{q}}$) を再構築し、最終的な検証式に代入して、すべてが一致するかを確認します。これにより、鍵全体を知らなくても、秘密の設計図が正しく存在したことだけが証明されます。

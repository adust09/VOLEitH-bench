## 関連研究 (Related Work)

VOLE-in-the-Head（VOLEitH）は、耐量子署名スキーム FAEST の中核をなすZKP技術であり、二つの主要な先行研究パラダイム、すなわち**MPC-in-the-Head (MPCitH)** と**QuickSilver**に深く関連しています。VOLEitHは、これらのアプローチの長所を取り入れ、効率的かつ公開検証可能な証明システムを実現しています。

### 1. MPC-in-the-Head (MPCitH) フレームワーク

**MPCitH**は、VOLEitHの背景にある最も重要な先行研究の一つであり、VOLEitHが改良を目指したZKPパラダイムです。

#### 1.1 MPCitHの起源と概念
MPCitH（Multi-Party Computation in the Head）は、2007年にIshai、Kushilevitz、Ostrovsky、Sahaiによって提案された手法であり、**セキュアな多人数計算（MPC）プロトコルを識別スキームやゼロ知識知識証明（ZKPoK）に変換する**ことを目的としています。

MPCitHの基本的な考え方は、証明者 ($P$) が秘密の証拠 ($x$) を複数のパーティのシェア ($\left[\![x]\right]_1, \ldots, \left[\![x]\right]_N$) に分解し、仮想的なMPCプロトコルを実行することです。検証者 ($V$) は、ランダムに選ばれたパーティ $i^*$ を除くすべてのパーティのビューの開示を要求することで、証拠の知識を確認します。

#### 1.2 MPCitHに基づく耐量子署名スキーム
MPCitHは、任意の**一方向関数** ($F: x \mapsto y$) に適用可能であるため、耐量子署名スキーム（PQS）を構築する上で非常に便利なツールとして利用されてきました。

*   **Picnic**: MPCitHに基づく有名なスキームとして、NISTの候補であった**Picnic**があります。
*   **対称鍵プリミティブ**: AES や、MPCフレンドリーなLowMC、Rain などの対称鍵プリミティブに基づくスキームが開発されています。
*   **他のPQC候補**: NISTの追加署名公募では、AIMer、Biscuit、MIRA、MiRitH、MQOM、PERK、RYDE、SDitHといった多くのMPCitHスキームが提出されています。
*   **FAESTとの関係**: FAESTは、Picnic や Banquet などの他のPQC署名アルゴリズムと同様に、秘密鍵の知識の非対話型議論から導出されますが、**MPCitHフレームワークでは構築されていません**。VOLEitHは、MPCitHアプローチと比較して、**よりシンプルで、より小さく、より高速な**プロトコルとなり得る点で、MPCitHパラダイムの限界を克服することを目指しています。

### 2. QuickSilver: VOLEベースのゼロ知識証明システム

VOLEitHは、その効率性を達成するために、VOLE（Vector Oblivious Linear Evaluation）に基づく特定のZKPシステムを応用しています。

#### 2.1 QuickSilverの構造と特徴
**QuickSilver**プロトコルは、Yang、Sarkar、Weng、Wangによる研究 [YSWW21] によって提案された**VOLEハイブリッドモデル**における**対話型ゼロ知識知識証明プロトコル**です。

*   **基盤**: QuickSilverは、Dittmerらによる**Line-Point Zero-Knowledge (LPZK) パラダイム**に基づいており、任意の有限体上の算術回路に対して証明を行うために利用されます。
*   **VOLEとの関係**: QuickSilverは、VOLEプロトコルへのアクセスを前提としており、VOLEによって提供されるタプル ($\mathbf{u}_i, \mathbf{v}_i, \mathbf{q}_i$) が、グローバルキー $\mathbf{\Delta}$ の下でのメッセージ認証コード（MAC）として機能する**準同型コミットメント**として利用されます。
*   **乗算チェック**: QuickSilverプロトコルは、回路内の**乗算制約**（$\mathbf{w}_{\alpha} \cdot \mathbf{w}_{\beta} = \mathbf{w}_{\gamma}$）の証明に特に優れています。VOLE MACの線形準同型特性を利用して、検証者 ($V$) は乗算ゲートの正当性を効率的にチェックすることができます。

#### 2.2 VOLEitHとQuickSilverの統合
FAEST署名アルゴリズムを構築するために、VOLEitHをQuickSilver情報理論的証明システムと組み合わせることで得られる対話型議論システムが、**Fiat-Shamir変換**によって非対話型に変換され、Random Oracle Model (ROM) で安全性が証明されます。

VOLEitHは、QuickSilverのような**指定検証者（Designated-Verifier）**設定のVOLEベースのプロトコルを、公開検証可能なプロトコルに変換するための技術です。FAESTでは、QuickSilverプロトコルをAESアルゴリズムから導出された回路の証明に適用することが、主要な設計選択となっています。

QuickSilverスタイルのプロトコル（$\Pi_{2D\text{-Rep}}$）を**FOT-1̄ハイブリッドモデル**に設定し、これに**O2Cコンパイラ**を適用することで、FAESTが採用するNIZKスキーム ($\Pi_{\text{FAEST}}$) が得られます。

---
**補足**: VOLEitHは、SoftSpokenOT の技術を応用し、OTベースのZKPを公開検証可能なプロトコルに変換するコンパイラを通じて、QuickSilverのようなVOLEベースのZKPシステムを利用可能にしています。

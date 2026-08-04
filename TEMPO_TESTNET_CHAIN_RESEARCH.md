# Tempo Testnet & Mainnet - Kapsamli Zincir Verileri Arastirmasi

> Arastirma Tarihi: 4 Agustos 2026
> Hazirlayan: Claude Code Arastirma Araci
> Veri Kaynaklari: Tempo RPC, Blockscout Explorer, CryptoBriefing, The Defiant, Protos, The Block, Token Terminal, CoinDesk

---

## 1. Proje Ozeti

| Ozellik | Detay |
|---------|-------|
| **Proje Adi** | Tempo |
| **Tur** | Layer 1 Blockchain (EVM Uyumlu) |
| **Odak** | Stablecoin Odemeleri |
| **Kulucokalanlar** | Stripe & Paradigm |
| **Fonlama** | $500M Series A (Ekim 2025) |
| **Degerleme** | $5 Milyar |
| **Yatirimcilar** | Thrive Capital, Greenoaks, Sequoia Capital, Ribbit Capital, SV Angel |
| **Konsensus** | Simplex Consensus (Commonware) |
| **EVM Uyumlulugu** | Reth SDK uzerine kurulu, Osaka hardfork uyumlu |
| **Token Standardi** | TIP-20 (Stablecoin odakli) |
| **Lisans** | Apache 2.0 / MIT (Cift lisans) |

### Tasarim Ortaklari (Design Partners)
Anthropic, Coupang, Deutsche Bank, DoorDash, Lead Bank, Mercury, Nubank, OpenAI, Revolut, Shopify, Standard Chartered, Visa, Brex, Coastal, Cross River, Deel, Faire, Figure, Gusto, Kalshi, Klarna, Mastercard, Payoneer, Persona, Ramp, UBS

### Altyapi Ortaklari (40+)
MetaMask, Phantom, LayerZero, Across, Chainlink, Fireblocks, Tenderly, Alchemy, Dune Analytics, Allium, Artemis, Blockscout, Dynamic, Blockradar, Chainstack, dRPC, Conduit

---

## 2. Zaman Cizelgesi

| Tarih | Olay |
|-------|------|
| Eylul 2025 | Tempo ilk duyurusu |
| Ekim 2025 | $500M Series A fonlama turu |
| 9 Aralik 2025 | **Andantino Testnet** baslatildi (Chain ID: 42429) |
| 8 Ocak 2026 | **Moderato Testnet** baslatildi (Chain ID: 42431) |
| 8 Mart 2026 | Andantino testnet kaldirildi |
| **18 Mart 2026** | **Mainnet baslatildi** + Machine Payments Protocol (MPP) |
| Nisan 2026 | 10,000+ gunluk aktif kullanici |
| Mayis 2026 | 3.9M islem / 177,000 benzersiz adres |
| Temmuz 2026 | Robinhood Chain tarafindan gunluk kullanici sayisinda gerildi |

---

## 3. Testnet Teknik Verileri (Moderato - RPC'den Canli Cekilen)

### Zincir Bilgileri

| Metrik | Deger |
|--------|-------|
| **Chain ID** | 42431 |
| **Ag Adi** | Moderato Testnet |
| **RPC URL** | `https://rpc.moderato.tempo.xyz` |
| **WebSocket** | `wss://rpc.moderato.tempo.xyz` |
| **Explorer** | `https://explore.tempo.xyz` |
| **Zincir Baslangici** | 8 Ocak 2026, 17:47:52 UTC |
| **Toplam Blok Sayisi** | ~7,077,000+ (4 Agustos 2026 itibariyle testnet verisi Mart 2026'dan) |
| **Ortalama Blok Suresi** | ~0.663 saniye (~663ms) |
| **Saniyede Blok** | ~1.51 blok/saniye |
| **Gunluk Blok Uretimi** | ~130,272 blok/gun |
| **Validator Sayisi** | 4 (Tempo ekibi tarafindan isletiliyor) |
| **Gas Token** | Yok (USD stablecoin ile odeme) |

### Blok Basina Islem Sayisi (Tarihsel Ornekler)

| Blok | Tarih | Islem/Blok |
|------|-------|------------|
| Blok 1 | 8 Ocak 2026 | 1 |
| Blok 100,000 | 9 Ocak 2026 | 14 |
| Blok 1,000,000 | 14 Ocak 2026 | 52 |
| Blok 3,000,000 | 30 Ocak 2026 | 73 |
| Blok 5,000,000 | 15 Subat 2026 | 101 |
| Blok 6,000,000 | 23 Subat 2026 | 115 |
| Blok 7,077,000 | 4 Mart 2026 | ~115 (ortalama) |

### Son 10 Blok Ornegi (Canli RPC Verisi - Mart 2026)

| Blok | Islem Sayisi |
|------|-------------|
| 7,077,013 | 117 |
| 7,077,016 | 104 |
| 7,077,017 | 111 |
| 7,077,020 | 132 |
| 7,077,024 | 99 |
| 7,077,026 | 110 |
| 7,077,028 | 132 |
| 7,077,031 | 133 |
| 7,077,033 | 92 |
| 7,077,035 | 119 |
| **Ortalama** | **~115 islem/blok** |

### Tahmini Toplam Islem Sayisi (Testnet - Trapez Yaklasimi)

| Blok Araligi | Ort. Islem/Blok | Tahmini Islemler |
|--------------|----------------|-----------------|
| 0 - 100K | 8 | ~750,000 |
| 100K - 1M | 33 | ~29,700,000 |
| 1M - 3M | 62 | ~125,000,000 |
| 3M - 5M | 87 | ~174,000,000 |
| 5M - 6M | 108 | ~108,000,000 |
| 6M - 7.08M | 115 | ~123,859,025 |
| **TOPLAM** | | **~561.3 Milyon islem** |

### Mevcut Performans (Mart 2026 Sonu)

| Metrik | Deger |
|--------|-------|
| **Anlik TPS** | ~173 islem/saniye |
| **Gunluk Islem (mevcut oran)** | ~14.9 Milyon |
| **Gas Kullanimi (son blok)** | 29,988,834 |
| **Gas Limiti** | 500,000,000 |
| **Gas Kullanim Orani** | %6.00 |

---

## 4. Mainnet Verileri (18 Mart 2026 Sonrasi)

### Kullanici Metrikleri

| Tarih | Metrik | Deger |
|-------|--------|-------|
| 18 Mart 2026 | Mainnet lansmanı | - |
| 19 Mart 2026 | Gunluk yeni cuzdanlar (zirve) | **7,629** |
| ~22 Nisan 2026 | 5. hafta DAU | **~5,600** |
| 21 Nisan 2026 | Gunluk yeni cuzdanlar | **1,749** (zirveden %77 dusus) |
| Nisan sonu 2026 | DAU | **10,000+** (aylik %100+ buyume) |

### Islem ve Hacim Verileri

| Metrik | Deger | Tarih |
|--------|-------|-------|
| **Toplam Islem (Mainnet)** | 3.9 Milyon | ~28 Mayis 2026 |
| **Benzersiz Adres** | 177,000 | ~28 Mayis 2026 |
| **Gercek TPS** | ~2.5 TPS | Nisan 2026 |
| **Bitcoin TPS (karsilastirma)** | ~5 TPS | - |
| **Hedeflenen TPS** | 10,000-100,000 TPS | (teknik kapasite) |

### Finansal Metrikler

| Metrik | Deger |
|--------|-------|
| **Gunluk Ucretler** | $205 |
| **TVL (Kilitli Toplam Deger)** | $3 Milyon |
| **Stablecoin Arzı** | $25M+ |
| **Gunluk Token Transfer Hacmi** | Zirveden 2/3 dusus ($9M -> ~$3M) |
| **DEX 24 Saat Hacmi** | $56,000 (zirveden %95 dusus) |
| **Gunluk DEX Islem Sayisi** | <2,000 (zirveden %77 dusus) |

### Stablecoin Dagilimlari (Mainnet)

| Stablecoin | Dolasimdaki Arz |
|------------|----------------|
| pathUSD | $8.2M |
| USDB | $4.5-5.5M |
| USDT0 | $4.5-5.5M |
| USDC.e (Stargate) | $4.5-5.5M |
| EURC.e (Stargate) | $4.5-5.5M |
| **Toplam** | **$25M+** |

### Kontrat Dagitim Istatistikleri

| Tarih | Gunluk Kontrat Dagitimi |
|-------|------------------------|
| 19 Mart 2026 (zirve) | 3,060 |
| 21 Nisan 2026 | 863 (%72 dusus) |

---

## 5. Cuzdanlar Basina Islem Analizi

### Tahminler (Mainnet - Mayis 2026 verisiyle)

| Hesaplama | Sonuc |
|-----------|-------|
| Toplam islem | 3,900,000 |
| Benzersiz adres | 177,000 |
| **Ort. islem/cuzdan** | **~22 islem** |

Bu ortalama, dagilimdaki carpiklik nedeniyle yaniltici olabilir:
- Bot/kontrat adresleri: Binlerce islem
- Airdrop avcilari: 50-200 islem
- Tek seferlik kullanicilar: 1-5 islem

---

## 6. Karsilastirmali Analiz

### Tempo vs Rakipler (Temmuz 2026)

| Metrik | Tempo | Robinhood Chain | Arc (Circle) |
|--------|-------|-----------------|-------------|
| **Lansman** | 18 Mart 2026 | 1 Temmuz 2026 | TBA |
| **Tur** | L1 | L2 (Arbitrum) | L1 |
| **DAU** | ~10,000 | 50,000+ (1 hafta) | - |
| **Gunluk Islem** | Dusuk | 1.7M | - |
| **DEX Hacmi** | $56K | $400-570M | - |
| **Gunluk Gelir** | $205 | $39,000 | - |
| **Yatirim** | $500M | - | - |
| **Odak** | B2B Odemeler | Perakende/DeFi | Stablecoin |

---

## 7. Teknik Ozellikler

### Testnette Test Edilebilir Ozellikler

1. **Ozel Odeme Seritleri (Dedicated Payment Lanes):** Odemeler protokol seviyesinde garantili blok alanina sahip
2. **Stablecoin-Native Gas:** Ucretler dogrudan USD stablecoin ile odenir
3. **Dahili Stabil Varlik DEX:** Stablecoin donusumleri icin optimize edilmis yerlesik DEX
4. **Odeme Metadata:** Fatura numarasi, maliyet merkezi gibi yapilandirilmis memo alanlari
5. **Hizli Finalite:** BFT konsensus ile ~0.5 saniyede blok finalitesi
6. **Modern Cuzdan Ozellikleri:** Gas sponsorlugu, toplu islemler, zamanlanmis odemeler, passkey kimlik dogrulama
7. **Machine Payments Protocol (MPP):** AI ajanlari ve makineler arasi otomatik odeme standardi

### Gelistirici Araclari

- **SDK'lar:** TypeScript, Rust, Go, Foundry
- **Uyumlu Araclar:** Foundry, Hardhat, Remix, ethers.js, web3.js, web3.py
- **GitHub:** github.com/tempoxyz/tempo (995 yildiz, 321 fork, 2,924 commit)

### Node Gereksinimleri

| Bileşen | Minimum |
|---------|---------|
| CPU | 16 cekirdek |
| RAM | 32 GB |
| Depolama | 1000 GB NVMe |
| Ag | 1 Gbps |

---

## 8. Airdrop Durumu ve Testnet Katilim

### Mevcut Durum
- **Resmi token veya airdrop duyurusu YOKTUR**
- Testnet katilimi tamamen spekulatiftir
- "Yerel token ve tesvik katmani yok" (resmi aciklama)
- Testnet tamamen teknik, test ve altyapi deneyimi icin

### CryptoRank Verileri
- Tempo airdrop kılavuzu: **10,889 goruntulenme**, 218 katilimci puani

### Testnet Aktiviteleri (Potansiyel Airdrop icin)
1. Cuzdan ekle (MetaMask'a Tempo Testnet agini ekle)
2. Faucet'ten test token al (AlphaUSD, BetaUSD, ThetaUSD vb.)
3. Zincir uzerinde islem gonder (onchaingm uzerinden)
4. Akilli kontrat dagit
5. Ekosistem dApp'lerini kesfet
6. Domain mint et
7. Node calistir (ileri duzey)

### Maliyet ve Sure
- **Gercek para gerektirmez** - sadece testnet tokenlari
- Baslangic: 15-30 dakika
- Aktif kalma: Gunluk birkaC dakika

---

## 9. Riskler ve Elestiriler

### Ana Elestiriler
1. **Dusuk Gercek TPS:** $500M yatirimla Bitcoin'den dusuk TPS (~2.5 vs ~5)
2. **Merkezilestirme:** Sadece 4 validator, hepsi Tempo ekibi tarafindan isletiliyor
3. **Dusuk Kullanici Benimsamesi:** B2B odakli model perakende buyumeyi yavaslatıyor
4. **Dusuk DEX Hacmi:** $56K gunluk hacim (global DEX hacminin %0.001'i)
5. **Gelir:** Gunluk sadece $205 ucret geliri
6. **Kullanici Kaybi:** Yeni cuzdan olusturma %77, kontrat dagitimi %72 dusus gosterdi

### Olumlu Yonler
1. Guclu kurumsal ortakliklar (Visa, Mastercard, Deutsche Bank, UBS)
2. $500M fonlama, $5B degerleme
3. Stripe destegi (fintech devi)
4. Machine Payments Protocol (AI ajanlar icin yenilikci)
5. 40+ altyapi ortagi
6. Acik kaynak, cift lisansli
7. $25M+ stablecoin arzi

---

## 10. Ozet Istatistikler Tablosu

| Metrik | Testnet (Moderato) | Mainnet |
|--------|-------------------|---------|
| **Baslangic** | 8 Ocak 2026 | 18 Mart 2026 |
| **Chain ID** | 42431 | - |
| **Toplam Blok** | ~7M+ | - |
| **Blok Suresi** | ~0.66 saniye | ~0.5 saniye |
| **Tahmini Toplam Islem** | ~561M (testnet) | 3.9M (2 ay) |
| **Benzersiz Adres** | Bilinmiyor | 177,000 |
| **DAU** | Bilinmiyor | ~10,000+ |
| **Gercek TPS** | ~173 (testnet) | ~2.5 (mainnet) |
| **Validator Sayisi** | 4 | Genisletiliyor |
| **Gas Token** | Yok (USD) | Yok (USD) |
| **TVL** | - | $3M |
| **Stablecoin Arzi** | - | $25M+ |

---

## 11. Veri Kaynaklari

- Tempo Resmi: https://tempo.xyz
- Tempo Dokumantasyon: https://docs.tempo.xyz
- Tempo Explorer: https://explore.tempo.xyz
- Tempo GitHub: https://github.com/tempoxyz/tempo
- CryptoBriefing - DAU Verileri: https://cryptobriefing.com/tempo-daily-active-users-surpass-10k/
- The Defiant - Islem Verileri: https://thedefiant.io/converge/blockchains/tempo-l1-hits-3-9m-transactions-in-two-months
- Protos - TPS Analizi: https://protos.com/stripes-tempo-blockchain-raised-500m-has-lower-tps-than-bitcoin/
- The Block - Mainnet Lansman: https://www.theblock.co/post/394131/tempo-mainnet-goes-live-with-machine-payments-protocol-for-agents
- CoinDesk - Testnet Lansman: https://www.coindesk.com/tech/2025/12/09/stripe-backed-blockchain-tempo-starts-testnet-kalshi-mastercard-ubs-added-as-partners
- Chainalysis - Tempo Destegi: https://www.chainalysis.com/blog/tempo-automatic-token-coverage-march-2026/
- Token Terminal - Aktif Kullanicilar: https://tokenterminal.com/explorer/projects/tempo/metrics/user-dau
- CryptoRank - Airdrop Kilavuzu: https://cryptorank.io/drophunting/tempo-chain-activity1027

---

> **Not:** Testnet verileri dogrudan Tempo Moderato RPC endpoint'inden (`https://rpc.moderato.tempo.xyz`) canli olarak cekilmistir. Mainnet verileri ise guvenilir kripto medya kaynaklarindan derlenmistir. Tum veriler belirtilen tarihlere aittir ve zaman icinde degisebilir.

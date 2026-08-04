# Tempo Testnet - Kapsamli Zincir Veri Arastirmasi

> Arastirma Tarihi: 4 Mart 2026
> Veri Kaynagi: Tempo RPC (rpc.moderato.tempo.xyz), Blockscout Explorer, Web kaynaklari

---

## 1. Proje Ozeti

**Tempo**, Stripe ve Paradigm isbirligi ile gelistirilen, odemeler icin optimize edilmis bir Layer 1 blockchain'dir. Ekim 2025'te 5 milyar dolar degerleme ile 500 milyon dolar Series A yatirim aldi.

- **Kuruluslar:** Stripe + Paradigm
- **Yatirim:** $500M Series A (Greenoaks, Thrive Capital, Sequoia Capital, Ribbit Capital, SV Angel)
- **Degerleme:** ~$5B
- **Odak:** Stablecoin odemeleri, dusuk maliyet, aninda kesinlik

---

## 2. Testnet Zaman Cizelgesi

| Olay | Tarih |
|------|-------|
| Ilk testnet (Andantino) lansmani | 9 Aralik 2025 |
| Moderato testnet lansmani | 8 Ocak 2026 |
| Andantino kullanim disi | 8 Mart 2026 (planlanan) |
| Mainnet beklentisi | 2026 (tarih belirsiz) |

---

## 3. Teknik Ozellikler

| Ozellik | Deger |
|---------|-------|
| Chain ID (Moderato) | 42431 |
| Chain ID (Andantino - eski) | 42429 |
| RPC Endpoint | https://rpc.moderato.tempo.xyz |
| WebSocket | wss://rpc.moderato.tempo.xyz |
| Block Explorer | https://explore.tempo.xyz |
| Blockscout Explorer | https://scout.tempo.xyz |
| Hedef TPS | 100,000+ |
| Blok Suresi (hedef) | ~0.5 saniye |
| Blok Suresi (gercek) | ~0.66 saniye |
| Validator Sayisi | 4 (Tempo ekibi tarafindan yonetiliyor) |
| Gas Token | Yok (USD stablecoin ile odeme) |
| Islem Maliyeti Hedefi | <$0.001 (bir sentin onda biri) |
| Konsensus | Byzantine Fault Tolerant (BFT) |
| EVM Uyumlulugu | Evet (Solidity, Foundry, Hardhat, Remix) |

---

## 4. Canli Zincir Verileri (RPC'den Cekilmistir)

### 4.1 Genel Istatistikler

| Metrik | Deger |
|--------|-------|
| Zincir Baslangici (Block 1) | 8 Ocak 2026, 17:47:52 UTC |
| Guncel Blok Numarasi | ~7,077,000 |
| Zincir Yasi | 54 gun (~7.7 hafta) |
| Ortalama Blok Suresi | 0.66 saniye |
| Gunluk Blok Uretimi | ~130,271 blok/gun |
| Tahmini Toplam Islem | ~561 Milyon+ |
| Gunluk Islem Orani (guncel) | ~15 Milyon islem/gun |

### 4.2 Blok Basina Islem Sayisi - Buyume Analizi

Farkli donemlerden alinan ornekler:

| Donem | Blok | Tarih | Islem/Blok |
|-------|------|-------|------------|
| Zincir Baslangici | #1 | 8 Ocak 2026 | 1 |
| Erken Donem | #100,000 | 9 Ocak 2026 | 14 |
| 1. Hafta | #1,000,000 | 14 Ocak 2026 | 52 |
| 3. Hafta | #3,000,000 | 30 Ocak 2026 | 73 |
| 5. Hafta | #5,000,000 | 15 Subat 2026 | 101 |
| 6. Hafta | #6,000,000 | 23 Subat 2026 | 115 |
| 8. Hafta (Guncel) | #7,077,000 | 4 Mart 2026 | 92-133 (ort. ~115) |

**Buyume Trendi:**
- Ocak basi: ~14 tx/blok
- Ocak ortasi: ~52 tx/blok (3.7x artis)
- Subat ortasi: ~101 tx/blok (1.9x artis)
- Mart basi: ~115 tx/blok (stabil)
- **Baslangictan bu yana ~8x buyume**

### 4.3 Guncel Blok Detaylari (4 Mart 2026)

Son 10 bloktan alinan ornekler:

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

- **Ortalama:** ~115 islem/blok
- **Minimum:** 92
- **Maksimum:** 133
- **Gas Kullanimi:** ~6% (Gas Limit: 500M, Kullanilan: ~30M)

---

## 5. Kullanici ve Cuzdan Verileri

### 5.1 Kamuya Acik Veriler

Tempo, testnet icin resmi kullanici veya cuzdan sayisi ACIKLAMAMISTIR. Ancak mevcut verilerle tahmin yapilabilir:

| Metrik | Deger | Kaynak |
|--------|-------|--------|
| CryptoRank Sayfa Goruntulenme | 10,889 | CryptoRank airdrop sayfasi |
| CryptoRank Topluluk Puani | 218 katilimci | CryptoRank |
| Tasarim Ortaklari | 25+ sirket | Resmi blog |
| Altyapi Ortaklari | 40+ | Resmi blog |
| Validator Sayisi | 4 | Resmi dokumantasyon |

### 5.2 Aktif Cuzdan Tahmini

Dogrudan RPC'den cuzdan sayisi cekilemez, ancak gunluk ~15M islem yapildigini biliyoruz. Testnet'lerde tipik olarak:

- **Airdrop avcisi/bot orani:** Testnet'lerde islemlerin buyuk cogunlugu otomasyon/bot kaynakli olabilir
- **Benzesim karsilastirmasi:** Benzer profildeki testnet'ler (Aptos, Sui, Base) lansmanlarinda 100K-500K arasi benzersiz adres gormustur
- **Tempo icin tahmini aralik:** 50,000 - 200,000 benzersiz adres (dogrulanmamis tahmin)

### 5.3 Cuzdan Basina Islem Tahmini

~561M toplam islem ve tahmini 50K-200K kullanici ile:
- **Dusuk senaryo (200K kullanici):** ~2,800 islem/cuzdan
- **Yuksek senaryo (50K kullanici):** ~11,200 islem/cuzdan
- **Not:** Bu rakamlar bot/otomasyon aktivitesini icerir, gercek kullanici basina ortalama muhtemelen daha dusuktur

---

## 6. Ekosistem ve Ortaklar

### 6.1 Tasarim Ortaklari (Design Partners)

| Kategori | Sirketler |
|----------|-----------|
| Fintech/Odeme | Stripe, Mastercard, Visa, Klarna, Payoneer, Brex, Ramp, Gusto, Deel |
| Bankacilik | Deutsche Bank, UBS, Standard Chartered, Nubank, Revolut, Mercury, Lead Bank, Cross River, Coastal |
| Teknoloji | OpenAI, Anthropic, Shopify, Coupang, DoorDash, Faire |
| Finans Platformu | Kalshi, Figure |
| Dijital Banka | Persona |

### 6.2 Altyapi Ortaklari (40+)

| Kategori | Ornekler |
|----------|----------|
| Cuzdan | MetaMask, Phantom, Dynamic, Blockradar |
| Interoperability | LayerZero, Across |
| Oracle | Chainlink |
| Analytics | Artemis, Allium |
| Gelistirici Araclari | Tenderly, Alchemy, Chainstack, dRPC, Conduit |
| DeFi | Frax, ve diger stablecoin projeleri |
| Faucet | Chainlink Faucets |

### 6.3 Onemli Gelismeler

- **Klarna USD:** Klarna, Tempo uzerinde KlarnaUSD stablecoin'ini cikardi - bir dijital bankanin Tempo uzerindeki ilk stablecoin'i
- **Node Calistirma:** Herkes node calistirabilir (Gereksinimler: 16 CPU, 32GB RAM, 1TB NVMe, 1Gbps ag)
- **Stablecoin Olusturma:** Kullanicilar tarayicidan kendi stablecoin'lerini olusturabilir

---

## 7. Testnet Katilim Rehberi

Testnet'e katilmak icin yapilabilecek islemler:

1. **Cuzdan Ayarlari:** MetaMask'a Tempo testnet ekle (Chain ID: 42431, RPC: https://rpc.moderato.tempo.xyz)
2. **Faucet Token Al:** https://faucets.chain.link/tempo-testnet adresinden test token'lari talep et
3. **Islem Gonder:** onchaingm.com uzerinden zincir uzerinde mesaj gonder
4. **Akilli Sozlesme Deploy Et:** Foundry/Hardhat ile akilli sozlesme yayinla
5. **Domain Satin Al:** Tempo test aginda domain al
6. **Stablecoin Olustur:** Tarayicidan ozel stablecoin olustur
7. **Node Calistir (ileri):** Tam node calistirarak ag dogrulamasina katil

**Maliyet:** Ucretsiz (sadece test token'lari kullanilir)
**Sure:** Baslangic ~15-30 dakika, sonra gunluk birkac dakika

---

## 8. Rakip Karsilastirma

| Ozellik | Tempo | Circle Arc | Ethereum L2'ler |
|---------|-------|------------|-----------------|
| Odak | Odeme | Stablecoin | Genel amacli |
| TPS Hedefi | 100K+ | Bilinmiyor | 1K-10K |
| Blok Suresi | ~0.5s | Bilinmiyor | 2-12s |
| Gas Token | Yok (USD) | Bilinmiyor | ETH |
| Yatirim | $500M | Bilinmiyor | Degisken |
| Ortaklar | Stripe, Visa, Mastercard | Goldman Sachs, BlackRock, Visa | Degisken |

---

## 9. Riskler ve Uyarilar

1. **Merkezilestirme:** Sadece 4 validator, hepsi Tempo ekibi tarafindan yonetiliyor
2. **Token Belirsizligi:** Resmi token veya airdrop duyurusu YOK
3. **Testnet Verisi:** Tum veriler testnet uzerindedir, mainnet performansi farkli olabilir
4. **Ucuncu Parti Uygulamalar:** Testnet'teki bazi uygulamalar resmi degil, DYOR
5. **Validator Gecisi:** Izinsiz validator setine gecis zaman alacak
6. **Ekonomik Model:** Uzun vadeli surdurulebilirlik garanti degil
7. **Bot Aktivitesi:** Testnet islemlerinin onemli bir kismi bot/otomasyon kaynakli olabilir

---

## 10. Veri Toplama Metodolojisi

Bu arastirmada kullanilan yontemler:

1. **RPC Dogrudan Sorgu:** Tempo Moderato testnet RPC endpoint'ine (https://rpc.moderato.tempo.xyz) dogrudan JSON-RPC cagrilari yapilarak blok verileri cekilmistir
2. **Blok Orneklemesi:** 10 farkli zaman diliminden bloklar orneklenerek islem yogunlugu analiz edilmistir
3. **Web Arastirmasi:** Resmi Tempo dokumantasyonu, blog yazilari, haber kaynaklari ve sosyal medya taranmistir
4. **Tahmini Hesaplamalar:** Toplam islem sayisi trapezoidal yaklasim ile hesaplanmistir
5. **Explorer Verileri:** Blockscout explorer (explore.tempo.xyz) uzerinden mevcut veriler kontrol edilmistir

**Not:** Benzersiz adres sayisi, cuzdan basina islem dagilimi gibi detayli metrikler RPC uzerinden dogrudan elde edilemez. Bu veriler icin Tempo'nun explorer API'si veya analytics platformlari (Artemis, Allium) kullanilmalidir.

---

## 11. Faydali Baglantilar

- Resmi Site: https://tempo.xyz
- Dokumantasyon: https://docs.tempo.xyz
- Block Explorer: https://explore.tempo.xyz
- Blockscout Explorer: https://scout.tempo.xyz
- GitHub: https://github.com/tempoxyz/tempo
- Faucet: https://faucets.chain.link/tempo-testnet
- Testnet Blog: https://tempo.xyz/blog/testnet
- RPC Endpoint: https://rpc.moderato.tempo.xyz

---

## 12. Ozet Dashboard

```
+--------------------------------------------------+
|         TEMPO TESTNET (Moderato) DASHBOARD        |
+--------------------------------------------------+
| Zincir Yasi:          54 gun (8 Ocak 2026'dan)   |
| Toplam Blok:          ~7,077,000                  |
| Tahmini Toplam TX:    ~561 Milyon+                |
| Gunluk TX (guncel):   ~15 Milyon                  |
| Ort. TX/Blok:         ~115                        |
| Blok Suresi:          ~0.66 saniye                |
| Gas Kullanimi:        ~%6                         |
| Validator:            4                           |
| Tasarim Ortaklari:    25+ sirket                  |
| Altyapi Ortaklari:    40+ sirket                  |
| Yatirim:              $500M @ $5B degerleme       |
+--------------------------------------------------+
```

# Phase 1: Windows Kabuğu ve Platform Altyapısı - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-10-18
**Phase:** 01-windows-kabugu-ve-platform-altyapisi
**Areas discussed:** Dağıtım ve güncelleme, Faz 1 penceresi ve bildirim, Tepsi ve arka plan, Anahtar kasası deneyimi

> Not: Oturum tartışma sırasında yeniden başladı; "Dağıtım ve güncelleme" alanı checkpoint'ten geri yüklendi (seçenek açıklamaları kaydedilmemişti).

---

## Dağıtım ve güncelleme

### Dağıtım kanalı

| Option | Selected |
|--------|----------|
| NSIS + GitHub güncelleyici | ✓ |
| Microsoft Store (MSIX) |  |
| Sen karar ver |  |

**User's choice:** NSIS + GitHub Releases (tauri-plugin-updater); Store v2'de yeniden değerlendirilir

### Yeni sürüm davranışı

| Option | Selected |
|--------|----------|
| İndir, sonra sor |  |
| Sessizce kur |  |
| Sadece elle denetle |  |

**User's choice:** Arka planda indir, sonra 'yeniden başlat' diye sor (bildirim + tepsi menüsü)

### Yayın yeri

| Option | Selected |
|--------|----------|
| Ayrı açık sürüm reposu |  |
| Bu repo zaten açık |  |
| Sen karar ver |  |

**User's choice:** Bu repo (emsal66/tempo) herkese açık; GitHub Releases doğrudan burada

### Sürüm tetikleyicisi

| Option | Selected |
|--------|----------|
| Etiketle, elle |  |
| Her main push'u |  |

**User's choice:** Her push'ta CI artifact; gerçek sürüm yalnızca v0.x.y etiketiyle

### İmza anahtarı yedeği

| Option | Selected |
|--------|----------|
| Şifre yöneticisi + USB | ✓ |
| Sadece şifre yöneticisi |  |
| Sonra karar veririm |  |

**User's choice:** Şifre yöneticisi + çevrimdışı USB + CI secret

---

## Faz 1 penceresi ve bildirim

### Faz 1 sonunda pencerede ne görünsün

| Option | Selected |
|--------|----------|
| Sade iskelet | ✓ |
| Sadece ayarlar ekranı |  |
| Sen karar ver |  |

**User's choice:** Sade iskelet: Mica cam arka plan + özel başlık çubuğu + Mornfold logosu + sistem durumu (son kontrol, sürüm) + Ayarlar; Faz 2'de gerçek pano gelir

### Elle başlatıldığında açılış davranışı

| Option | Selected |
|--------|----------|
| Pencere açılır | ✓ |
| Her zaman tepsiye |  |
| Mini panel |  |

**User's choice:** Pencere açılır; Windows açılışında otomatik başlayınca sadece tepsiye iner

### Windows bildirim stili

| Option | Selected |
|--------|----------|
| Butonlu | ✓ |
| Sade |  |

**User's choice:** Butonlu: tıklayınca ilgili ekran açılır + 1-2 hızlı buton (Aç, Sessize al); uygulama kapalıyken de tıklama çalışır

### Bildirim sesi

| Option | Selected |
|--------|----------|
| Sessiz varsayılan |  |
| Windows varsayılanı |  |
| Özel Mornfold sesi | ✓ |

**User's choice:** Özel Mornfold sesi; referans: Discord bildirim sesi karakteri (kısa, yumuşak, tanınabilir 'ping'). Discord'un sesi birebir kopyalanmaz (telif) — benzer hissi veren özgün bir ses üretilir/lisanslanır

### Ses hangi bildirimlerde çalsın

| Option | Selected |
|--------|----------|
| Hepsinde, kapatılabilir |  |
| Sadece önemlilerde |  |

**User's choice:** Hepsinde; Ayarlar'dan tamamen veya servis bazında kapatılabilir; sessiz saatlerde çalmaz

---

## Tepsi ve arka plan

### Tepsi ikonuna sol tık

| Option | Selected |
|--------|----------|
| Pencereyi aç/gizle |  |
| Menü açılsın |  |

**User's choice:** Sol tık pencereyi açar/gizler; sağ tık menü (Aç / Tümünü duraklat / Çık) — Discord gibi

### Tepsi ikonu durum göstersin mi

| Option | Selected |
|--------|----------|
| Rozet + durum |  |
| Sade ikon |  |

**User's choice:** Okunmamış bildirim rozeti; duraklatıldığında/hata olduğunda ikon soluk/farklı görünür

### Tümünü duraklat davranışı

| Option | Selected |
|--------|----------|
| Süre seçmeli | ✓ |
| Aç/kapa |  |

**User's choice:** Süre seçmeli: 1 saat / Yarına kadar / Ben açana kadar (Discord 'rahatsız etme' gibi)

### X düğmesi ilk kapatma

| Option | Selected |
|--------|----------|
| Tepsiye in + bir kez bilgi ver |  |
| Sessizce tepsiye in | ✓ |
| Ayarlardan seçilsin |  |

**User's choice:** Sessizce tepsiye iner, bilgi bildirimi yok

---

## Anahtar kasası deneyimi

### Kayıtlı anahtar görünümü

| Option | Selected |
|--------|----------|
| Maskeli + son 4 hane | ✓ |
| Sadece 'Kayıtlı ✓' |  |

**User's choice:** Maskeli + son 4 hane (örn. 'sk-ant-••••a1b2'); değiştir / sil / test et butonları

### Değiştirme/silmede ek doğrulama

| Option | Selected |
|--------|----------|
| Windows Hello |  |
| Gerek yok | ✓ |

**User's choice:** Gerek yok; Windows oturumu açıkken doğrudan değiştirilebilir

### Anahtar girildiğinde

| Option | Selected |
|--------|----------|
| Test et, sonra kaydet |  |
| Direkt kaydet |  |

**User's choice:** Kaydetmeden önce test isteği, geçersizse uyar (test yalnızca sağlayıcısı bilinen anahtarlar için; genel anahtarlarda test atlanabilir)

### Faz 1 kasa kapsamı

| Option | Selected |
|--------|----------|
| Genel kasa ekranı |  |
| Sadece AI anahtarları |  |

**User's choice:** Ayarlar'da genel 'Anahtarlar' bölümü: ad + değer ekle/sil/test; AI sağlayıcı alanları Faz 5'te buraya bağlanır

---

## Claude's Discretion

- Geçici iskelet ekranın yerleşimi/tipografisi, bildirim buton metinleri, tepsi rozet görselleri, zamanlayıcı test işi, AEAD crate seçimi.

## Deferred Ideas

- Microsoft Store (MSIX) kanalı — v2
- Sessiz saatler kuralı — Faz 3

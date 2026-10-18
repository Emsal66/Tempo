# Faz 1 Smoke Testi — Windows 11 kontrol listesi

## Amaç

Bu liste, Linux geliştirme ortamında ve CI'da doğrulanamayan Windows'a özgü davranışları
(Mica cam, tepsi, otomatik başlatma, bildirimler, ses, uyku/uyanma, Kimlik Bilgisi Yöneticisi,
güncelleyici) **senin gerçek Windows 11 PC'nde** doğrulamak içindir. Her madde hangi gereksinimi
veya kararı kanıtladığını belirtir.

## Test edilen sürüm

> Bu bölümü plan 01-18 doldurur.

- Etiket: 
- Etiketlenen commit SHA: 
- CI artifact adı: 
- Sürüm (release) adresi: 

## Hangi kurulum dosyası hangi madde için?

- **S-0:** kurulum gerekmez.
- **S-1 … S-9:** `v0.1.0` etiketli commit'in **her push'ta üretilen CI artifact'ı**.
  GitHub → **Actions** → o SHA'nın CI çalıştırması → artifact **`mornfold-nsis-<sha>`** (zip).
  Bu, ROADMAP başarı kriteri 1'i ("her push bir Windows kurulum dosyası üretir") birebir kanıtlar.
  (Geliştirme konteyneri artifact'ı indiremez; proxy `/actions/*` yollarını engeller. Bu yüzden
  indirme adımını sen yaparsın.)
- **S-10:** yayımlanmış **v0.1.0 sürümünün kurulum dosyası**: GitHub Releases'tan
  `Mornfold_0.1.0_x64-setup.exe`.
- **S-11:** kurulum gerekmez.

> **Beklenen durum**
>
> S-1 … S-9 çalışırken v0.1.0 sürümü hâlâ **taslak** (draft) durumdadır; yalnızca bu kontroller
> geçtikten sonra yayımlanır. Bu yüzden güncelleyici yayımlanmış bir `latest.json` bulamaz ve
> Durum / Güncellemeler alanında **"Güncelleme denetlenemedi: …"** görünebilir. Bu **beklenen bir
> durumdur** ve S-1 … S-9 için **hata sayılmaz**. Güncellemeleri yalnızca S-10 test eder.

## Sonuçları nasıl bildireceksin?

1. Sondaki **Sonuç tablosunu** kopyala.
2. Her satırın "Sonuç" hücresine ✅ (geçti) veya ❌ (geçmedi) yaz; "Not" hücresine gözlemini ekle
   (❌ ise ne gördüğünü, mümkünse ekran görüntüsü).
3. Tablonun altındaki iki satırı (ses stratejisi ve soğuk açılış) doldur.
4. Hepsini yürütücüye (sohbete) geri yapıştır.

---

## S-0 — Akıllı Uygulama Denetimi durumu

**Kanıtladığı:** SHELL-01 (imzasız kurulumun çalışabilmesi), ADR 0001

1. **Ayarlar → Gizlilik ve güvenlik → Windows Güvenliği → Uygulama ve tarayıcı denetimi →
   Akıllı Uygulama Denetimi** sayfasını aç.
2. Durumu not et: **Açık**, **Değerlendirme** veya **Kapalı**.

**Beklenen:** "Değerlendirme" veya "Kapalı".

- **"Açık" ise: dur ve bildir.** Akıllı Uygulama Denetimi imzasız kodu tamamen engeller; bu durumda
  kod imzalama engelleyici hale gelir ve kalan maddeler çalıştırılamaz.
- ⚠️ **Uyarı:** Akıllı Uygulama Denetimi'ni kapatırsan, Windows'u sıfırlamadan **yeniden
  açamazsın**. Kapatmadan önce yürütücüyle konuş.

## S-1 — CI artifact'ından kurulum

**Kanıtladığı:** SHELL-01, ROADMAP başarı kriteri 1, D-01, D-04

1. GitHub → **Actions** → "Test edilen sürüm" bölümündeki SHA'ya ait CI çalıştırmasını aç.
2. **Artifacts** altından `mornfold-nsis-<sha>` dosyasını (zip) indir.
3. Zip'i bir klasöre çıkar; içindeki NSIS `.exe` kurulum dosyasını çalıştır.
4. SmartScreen uyarısı çıkarsa: **Ek bilgi → Yine de çalıştır**.
5. Kurulum sihirbazını sonuna kadar ilerlet.

**Beklenen:**

- Kurulum sihirbazının dili **Türkçe**.
- **Yönetici izni (UAC) istenmez**; kurulum kullanıcı başına yapılır.
- Kurulum sonunda Başlat menüsünde **Mornfold** görünür.

## S-2 — İlk pencere

**Kanıtladığı:** D-06, SHELL-08

1. Mornfold'u Başlat menüsünden aç.
2. Pencereyi incele.

**Beklenen:**

- Arka planda **Mica** cam efekti (masaüstü renkleri hafifçe arkadan sızar).
- Windows'un standart başlık çubuğu yok; **özel başlık çubuğu** var: sürükleyerek taşınır,
  **simge durumuna küçült**, **ekranı kapla** ve **kapat** düğmeleri çalışır.
- **Mornfold logosu** görünür.
- **Durum** (son kontrol, sürüm, güncelleme durumu) ve **Ayarlar** bölümleri var.
- **Tüm metinler Türkçe.**

## S-3 — Tepsi davranışı ve duraklatma

**Kanıtladığı:** SHELL-02, D-11, D-12, D-13, D-14, ADR 0002 (A8)

1. Pencerenin **X** düğmesine bas.
   **Beklenen:** pencere sessizce tepsiye iner; **hiçbir bildirim çıkmaz** (D-14).
2. Tepsi simgesine **sol tıkla** → pencere açılır; tekrar sol tıkla → pencere gizlenir.
3. Tepsi simgesine **sağ tıkla**. **Beklenen menü:** **Aç** / **Tümünü duraklat ▸**
   (**1 saat** / **Yarına kadar** / **Ben açana kadar**) / **Çık**.
4. **Tümünü duraklat → 1 saat** seç.
   **Beklenen:** simge **soluklaşır**; menüde ve ipucunda **"devam: …"** (saat) görünür.
5. **Tümünü duraklat → Yarına kadar** seç. **Beklenen:** "devam: yarın 00:00".
6. Duraklatmayı kaldır (menüden devam ettir veya **Ben açana kadar** sonrası elle aç).
7. **Çık** seç. Görev Yöneticisi'ni (Ctrl+Shift+Esc) aç.
   **Beklenen:** **Mornfold.exe** işlemi **kalmamış**.
8. **Soru:** "Okunmamış bildirim rozeti simgede küçük bir nokta, sayı ise ipucunda
   ('2 okunmamış') gösteriliyor — bu görünüm sizin için uygun mu?"
   Cevabını sonuç tablosunun **Not** sütununa yaz.

## S-4 — Windows açılışında başlatma

**Kanıtladığı:** SHELL-03, D-07

1. Mornfold → **Ayarlar** → **"Windows açılışında başlat"** seçeneğini aç.
2. **Görev Yöneticisi → Başlangıç uygulamaları** sekmesinde **Mornfold**'un listelendiğini
   kontrol et.
3. PC'yi yeniden başlat ve oturum aç.
   **Beklenen:** Mornfold **yalnızca tepside** çalışır; **pencere açılmaz, ekranda yanıp sönmez**.
4. Ayarlar'dan **"Windows açılışında başlat"** seçeneğini kapat.
5. PC'yi yeniden başlat.
   **Beklenen:** Mornfold **başlamaz**.

## S-5 — Tek örnek

**Kanıtladığı:** SHELL-04

1. Mornfold çalışırken ve pencere açıkken Başlat menüsünden Mornfold'u **tekrar** aç.
   **Beklenen:** mevcut pencere **öne gelir**; Görev Yöneticisi'nde hâlâ **tek** Mornfold işlemi var.
2. Pencereyi X ile tepsiye gizle; Başlat menüsünden Mornfold'u tekrar aç.
   **Beklenen:** gizli pencere **görünür olur ve öne gelir**; yine tek işlem.

## S-6 — Bildirimler ve tıklama ile açılış

**Kanıtladığı:** SHELL-05, D-08, D-12

Ayarlar → **"Test bildirimi gönder"**.
**Beklenen:** bildirim başlığında **"Mornfold"** adı ve **Mornfold simgesi** görünür
(PowerShell değil).

- **(a) Sıcak tıklama:** uygulama çalışırken bildirim gövdesine tıkla.
  **Beklenen:** **Ayarlar** açılır.
- **(b) Soğuk tıklama:** tepsi menüsünden **Çık**; ardından **Win+N** ile Bildirim Merkezi'ni aç ve
  test bildirimine tıkla.
  **Beklenen:** uygulama başlar ve **Ayarlar** ekranında açılır.
- **(c) Sessize al:** yeni bir test bildirimi gönder ve **"Sessize al"** düğmesine bas.
  **Beklenen:** Ayarlar'da **"Sessiz · bitiş: …"** görünür; süre **1 saat**.
- **(c2) Okunmamış rozeti:** "Sessize al" süresini Ayarlar'dan kaldır ("Sesi şimdi aç"). Ayarlar →
  Bildirimler'de **"Test bildirimi gönder"**e bas ve hemen ardından pencereyi **X** ile tepsiye
  gizle (bildirime tıklama).
  **Beklenen:** pencere gizliyken tepsi simgesinde **okunmamış noktası** görünür, ipucunda
  **"1 okunmamış"** yazar. Tepsiye tıklayıp pencereyi aç → **nokta ve sayı kaybolur**.
- **(d) Dışarıdan gelen bağlantılar:** **Win+R** ile Çalıştır kutusunu aç ve
  `mornfold://act/mute?n=01HZZZZZZZZZZZZZZZZZZZZZZZ` yaz, Enter.
  **Beklenen:** **hiçbir şey sessize alınmaz** (bağlantı reddedilir).
  Ardından Win+R ile `mornfold://open/settings` yaz, Enter.
  **Beklenen:** **Ayarlar** açılır.

## S-7 — Bildirim sesi

**Kanıtladığı:** D-09, D-10, ADR 0002 (ses stratejisi)

1. Ayarlar → ses stratejisi **"Windows bildirimiyle çal"** seçiliyken test bildirimi gönder.
   **Beklenen:** **Mornfold ping'i** duyulur (Windows'un varsayılan bildirim sesi değil).
2. Duyulmazsa veya varsayılan Windows sesi çalarsa: stratejiyi **"Uygulama kendisi çalsın"** yap ve
   tekrar test et.
3. **Odak / Rahatsız etmeyin** modunu aç ve test bildirimi gönder.
   **Beklenen:** **hiç ses çalmaz**.
4. Rahatsız etmeyin'i kapat. Ayarlar'da **"Bildirim sesi"** seçeneğini kapat, test bildirimi gönder.
   **Beklenen:** bildirim **sessiz** gelir.
5. "Bildirim sesi"ni tekrar aç; **kaynak bazındaki** ses anahtarlarından birini kapatıp o kaynaktan
   test et. **Beklenen:** yalnızca kapattığın kaynak sessizdir.
6. Hangi stratejinin çalıştığını tablonun altındaki **"Ses stratejisi sonucu"** satırına yaz
   (A = Windows bildirimiyle, B = uygulama kendisi).

## S-8 — Uyku sonrası telafi

**Kanıtladığı:** SHELL-06, ADR 0002 (zamanlayıcı işleri)

1. Tepsi simgesinin ipucundaki **"son kontrol"** zamanını not et.
2. PC'yi **20 dakikadan uzun** süre uyku moduna al.
3. PC'yi uyandır ve oturum aç.

**Beklenen:**

- Yaklaşık **30 saniye içinde** ipucu ve Durum ekranı **yeni bir kontrol** zamanı gösterir.
- Durum → **"Son çalışmalar"** listesinde `heartbeat` ve `demo_check` **birer kez**, **"telafi"**
  işaretiyle ve **sırayla** görünür.
- Arka arkaya tekrarlanan çalışma **patlaması yok**.

## S-9 — Anahtar kasası

**Kanıtladığı:** SHELL-07, D-15, D-17, ADR 0002 (anahtar testleri)

- **(a) Sağlayıcısı tanınmayan anahtar:** Ayarlar → **Anahtarlar** → ad `test`, değer
  `ornek-anahtar-1234567890a1b2` ekle.
  **Beklenen:** **"Bu anahtarın sağlayıcısı tanınmadı; test atlandı."** mesajı çıkar, anahtar
  **kaydedilir** ve listede **`••••a1b2`** olarak görünür.
- **(b) Geçersiz Anthropic anahtarı:** ad `anthropic-deneme`, değer
  `sk-ant-api03-gecersiz00000000000000000000` ekle.
  **Beklenen:** önce **"Test ediliyor…"**, sonra **"Anahtar geçersiz: Anthropic bu anahtarı
  reddetti"**. **"Yine de kaydet"** basılmadıkça anahtar **kaydedilmez** — **"Vazgeç"** seç ve
  listede görünmediğini kontrol et.
- **(c) İsteğe bağlı — gerçek anahtar:** elinde gerçek bir Anthropic / Gemini / OpenRouter /
  OpenAI / GitHub / Slack anahtarı varsa ekle.
  **Beklenen:** **"Anahtar geçerli (…)"** ve anahtar maskeli kaydedilir (ör. `sk-ant-••••a1b2`).
- **(d) Kayıtlı anahtarı test et:** kayıtlı bir satırda **"Test et"** düğmesine bas.
  **Beklenen:** sonuç, değeri **yeniden yazmadan** görünür.
- **(e) Kalıcılık ve sızıntı kontrolü:**
  1. Uygulamayı tepsiden **Çık** ile kapat ve yeniden aç. **Beklenen:** anahtarlar hâlâ listede.
  2. **Denetim Masası → Kimlik Bilgisi Yöneticisi → Windows Kimlik Bilgileri → Genel Kimlik
     Bilgileri** altında Mornfold için **tam olarak bir** kayıt olmalı.
  3. PowerShell'de günlük ve veritabanı dosyalarında anahtar parçalarını ara:

     ```powershell
     $d = "$env:LOCALAPPDATA\com.mornfold.desktop"
     Get-ChildItem $d -Recurse -File -Include *.log,*.db,*.db-wal | Select-String -Pattern '1234567890a1b2','gecersiz0000' -SimpleMatch
     ```

     **Beklenen:** **hiç sonuç yok** (`%LOCALAPPDATA%\com.mornfold.desktop\logs` ve `.db` dosyası
     dahil).

## S-10 — Sürüm kurulumu ve güncelleme

**Kanıtladığı:** SHELL-09, D-02, D-03

1. Yayımlanmış **v0.1.0** sürümünden `Mornfold_0.1.0_x64-setup.exe` dosyasını indir ve CI artifact
   kurulumunun **üzerine** kur (kurulum soruyorsa **yeniden kur** seç; tanımlayıcı aynı olduğu için
   veriler ve anahtarlar korunur).
   **Beklenen:** Durum ekranında sürüm **0.1.0**.
2. **v0.2.0** yayımlandıktan sonra Durum'da **"Güncellemeleri denetle"**ye bas (veya 6 saate kadar
   bekle).
   **Beklenen:** tepsi menüsünde **"Yeniden başlat ve güncelle"** öğesi belirir ve bir **bildirim**
   çıkar (D-02).
3. Güncellemeyi kabul et.
   **Beklenen:** uygulama yeniden başlar ve **v0.2.0** olarak açılır (Durum sürümü gösterir);
   pencere görünür; **anahtarlar ve ayarlar korunmuştur**.

## S-11 — Güncelleyici anahtarı yedekleri

**Kanıtladığı:** D-05, ADR 0001

1. Şifre yöneticisinde güncelleyici anahtarı + parolası için bir kayıt var.
2. Çevrimdışı USB bellekte `mornfold.key` ve `mornfold.key.pub` dosyaları var.
3. GitHub → repo **Settings → Secrets and variables → Actions** altında
   `TAURI_SIGNING_PRIVATE_KEY` ve `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` görünüyor.

**Beklenen:** üç yedeğin üçü de mevcut. (Anahtarın veya parolanın kendisini sohbete **asla**
yapıştırma; yalnızca var/yok bildir.)

---

## Sonuç tablosu

| ID | Kontrol | Sonuç | Not |
|---|---|---|---|
| S-0 | Akıllı Uygulama Denetimi durumu |  |  |
| S-1 | CI artifact'ından kurulum (Türkçe, yönetici izni yok) |  |  |
| S-2 | İlk pencere (Mica, özel başlık çubuğu, Türkçe) |  |  |
| S-3 | Tepsi, menü, duraklatma, Çık (+ rozet görünümü sorusu) |  |  |
| S-4 | Windows açılışında başlatma (yalnızca tepsi) |  |  |
| S-5 | Tek örnek |  |  |
| S-6 | Bildirimler: sıcak/soğuk tıklama, sessize al, okunmamış, bağlantı reddi |  |  |
| S-7 | Bildirim sesi ve stratejisi |  |  |
| S-8 | Uyku sonrası telafi |  |  |
| S-9 | Anahtar kasası (atlanan test, geçersiz uyarısı, sızıntı yok) |  |  |
| S-10 | Sürüm kurulumu ve güncelleme |  |  |
| S-11 | Güncelleyici anahtarı yedekleri |  |  |

Ses stratejisi sonucu: ____ (A / B)

Bildirim Merkezi'nden soğuk açılış: çalıştı / çalışmadı

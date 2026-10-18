# Roadmap: Mornfold

## Overview

Mornfold aşağıdan yukarıya, katman katman inşa edilir. Önce Windows platform katmanı (kurulum, tepsi, bildirim, güvenli kasa, zamanlayıcı, güncelleme) gerçek bir Windows 11 kurulumunda kanıtlanır. Üzerine Apple / Linear zarafetinde tasarım sistemi ve kart kütüphanesi gelir. Ardından her servisin oturacağı bağlantı altyapısı, bildirim hattı ve kod gerektirmeyen API kartları kurulur. Bu üç katman Core Value'nun kendisidir: "zarif kabuk + servis bağlama altyapısı". Sonra ilk yerleşik servis (Google, salt okunur) ve AI olmadan çalışan "Günün özeti" ana ekranı, onun üstüne kaynaklara bağlı AI sabah özeti, ardından tüm yazma işlemlerini koruyan aksiyon hattı gelir. Son iki katman takip ve genişletilebilirliktir: web sitesi değişiklik takibi, MCP eklentileri ve başka kullanıcıların uygulamayı kendi başına kurabilmesi için ilk kurulum sihirbazı.

## Phases

**Phase Numbering:**

- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Windows Kabuğu ve Platform Altyapısı** - Kurulum, tepsi, tek kopya, otomatik başlatma, bildirim, güvenli kasa, uyku sonrası telafi, Türkçe altyapı ve imzalı güncelleme gerçek Windows 11'de çalışır
- [ ] **Phase 2: Tasarım Sistemi ve Kart Kütüphanesi** - Mica cam görünüm, temalar, kart sözleşmesi, bölüm ızgarası, detay paneli ve Ctrl+K paleti örnek verilerle hazır
- [ ] **Phase 3: Bağlantı Altyapısı, Bildirim Hattı ve API Kartları** - Bağlantılar sayfası, bildirim merkezi, sessiz ilk senkronizasyon ve kod yazmadan API kartı / katalog kartı ekleme
- [ ] **Phase 4: Google Bağlayıcısı ve Günün Özeti Ana Ekranı** - Gmail ve Takvim salt okunur bağlanır; AI olmadan çalışan "Günün özeti" ana ekranı gelir
- [ ] **Phase 5: AI Sabah Özeti** - Seçilebilir sağlayıcıyla, kaynaklara bağlı, işlem yapamayan, maliyeti görünür Türkçe sabah özeti
- [ ] **Phase 6: Aksiyon Hattı** - Mail yanıtlama / arşivleme, etkinlik ekleme ve özet aksiyon butonları; hepsi önizleme + onay + denetim günlüğü ile
- [ ] **Phase 7: Web Sitesi Değişiklik Takibi** - URL + seçici ile takip, kelime bazlı fark, koşullar, gürültü kuralları ve dürüst hata durumları
- [ ] **Phase 8: MCP Eklentileri ve İlk Kurulum** - Onaylı ve izole MCP sunucuları, MCP kartları ve yeni kullanıcı için atlanabilir kurulum sihirbazı

## Phase Details

### Phase 1: Windows Kabuğu ve Platform Altyapısı

**Goal**: Kullanıcı Mornfold'u Windows 11'e kurar ve uygulama arka planda güvenilir şekilde yaşar: tepside durur, tek kopya çalışır, bildirim gösterir, anahtarları güvenli kasada tutar, uykudan sonra kaçırdıklarını telafi eder ve kendini imzalı sürümle günceller
**Depends on**: Nothing (first phase)
**Requirements**: SHELL-01, SHELL-02, SHELL-03, SHELL-04, SHELL-05, SHELL-06, SHELL-07, SHELL-08, SHELL-09
**Success Criteria** (what must be TRUE):

  1. Kullanıcı Windows CI'ın her push'ta ürettiği NSIS kurulum dosyasıyla Mornfold'u Windows 11'e kurar; uygulamayı ikinci kez başlattığında yeni kopya açılmaz, mevcut pencere öne gelir
  2. Kullanıcı pencereyi kapattığında uygulama tepside kalır; tepsi menüsünden Aç / Tümünü duraklat / Çık çalışır; "Windows açılışında başlat" açıkken bilgisayar yeniden başlatıldığında uygulama tepside küçültülmüş olarak gelir
  3. Kurulu sürümden gönderilen bir test bildirimi Mornfold adı ve ikonuyla görünür ve tıklanınca uygulamada ilgili ekranı açar; bilgisayar uykudan uyandığında kaçırılan kontroller bir kez, sırayla çalışır ve tepside "son kontrol: X dk önce" satırı görünür
  4. Kullanıcının girdiği bir API anahtarı yeniden başlatmadan sonra da kullanılabilir, Windows Kimlik Bilgisi Yöneticisi'ne bağlı kasada şifreli durur, arayüzden geri okunamaz ve log dosyalarında görünmez; tüm arayüz metinleri Türkçe ve i18n altyapısından gelir
  5. Kullanıcı v0.1'den v0.2'ye uygulama içinden imzalı güncelleme alabilir; dağıtım kanalı kararı yazılı olarak alınmış ve güncelleme imza anahtarı yedeklenmiştir

**Plans**: 19 plans (10 waves)

Plans:
**Wave 1**

- [ ] 01-01-PLAN.md — Dev-env script, Cargo workspace, mornfold-core foundation (schema, DB, settings, shared contract types)
- [ ] 01-02-PLAN.md — UI scaffold (React/Vite/Tailwind/Biome/Vitest) + full Turkish i18n catalog and guards
- [ ] 01-03-PLAN.md — ADRs (distribution channel D-01, Phase 1 defaults), smoke checklist S-0..S-11, original ping + icons
- [ ] 01-04-PLAN.md — Owner steps: Smart App Control check, updater key generation + backups, public key recorded

**Wave 2** *(blocked on Wave 1 completion)*

- [ ] 01-05-PLAN.md — Core scheduler with sleep catch-up, gap detection, crash isolation + pause durations
- [ ] 01-06-PLAN.md — Core envelope-encrypted vault (AES-GCM, KCV, providers) + masking/redaction + canary test + D-17 provider key testers (reqwest, wiremock)
- [ ] 01-07-PLAN.md — Core toast XML builder, mornfold:// allowlist, single-use action tokens, sound policy
- [ ] 01-08-PLAN.md — Core i18n catalog over tr.json, relative-time labels, tray model (menu/tooltip/icon)
- [ ] 01-09-PLAN.md — Tauri shell skeleton: config (Mica, NSIS, scheme, CSP), single instance, close-to-tray, autostart, bindings

**Wave 3** *(blocked on Wave 2 completion)*

- [ ] 01-10-PLAN.md — CI: Linux suite + Windows build/test/startup smoke/NSIS artifact + ci-wait.sh gate, driven to green
- [ ] 01-11-PLAN.md — UI window skeleton: title bar, Durum/Ayarlar, Genel (autostart, logs)

**Wave 4** *(blocked on Wave 3 completion)*

- [ ] 01-12-PLAN.md — Vault in the shell: Credential Manager store, write-only secrets IPC (test-before-save, secrets_test_saved), log redaction, Anahtarlar UI

**Wave 5** *(blocked on Wave 4 completion)*

- [ ] 01-13-PLAN.md — Tray + background runtime: scheduler (initial kick), power resume, pause menu, safe shutdown, live Durum

**Wave 6** *(blocked on Wave 5 completion)*

- [ ] 01-14-PLAN.md — WinRT toasts with buttons and sound, mornfold:// activation (warm/cold), unread clearing

**Wave 7** *(blocked on Wave 6 completion)*

- [ ] 01-15-PLAN.md — Bildirimler UI + toast navigation handling (warm/cold)
- [ ] 01-16-PLAN.md — Signed updater: config gate, tag release workflow, non-blocking download-then-ask flow

**Wave 8** *(blocked on Wave 7 completion)*

- [ ] 01-17-PLAN.md — Güncellemeler UI + update state on Durum

**Wave 9** *(blocked on Wave 8 completion)*

- [ ] 01-18-PLAN.md — v0.1.0 release + owner smoke S-0..S-9 on Windows 11 (CI artifact of the tagged commit)

**Wave 10** *(blocked on Wave 9 completion)*

- [ ] 01-19-PLAN.md — v0.2.0 in-app update from the v0.1.0 release installer (S-10), key backups (S-11), validation sign-off

**Cross-cutting constraints:**

- The pushed HEAD is green on the linux and windows CI jobs

### Phase 2: Tasarım Sistemi ve Kart Kütüphanesi

**Goal**: Uygulama açıldığında "hayran bırakan" Apple / Linear tarzı arayüz görünür ve sonraki tüm servislerin kullanacağı sabit kart kütüphanesi, ızgara, detay paneli ve komut paleti örnek (fixture) verilerle çalışır
**Depends on**: Phase 1
**Requirements**: DSGN-01, DSGN-02, DSGN-03, DSGN-04, DSGN-05, DSGN-06
**Success Criteria** (what must be TRUE):

  1. Kullanıcı Mica cam arka planlı, özel başlık çubuklu, ince tipografili ve yumuşak animasyonlu bir pencere görür; açık / koyu tema ve "şeffaflığı azalt" arasında geçiş yaptığında görünüm anında değişir ve tercih kalıcıdır
  2. Her kart tipi (liste, ajanda, değer + mini grafik, zaman çizelgesi, fark, özet) yükleniyor / boş / hata / eski veri durumlarını, son güncelleme zamanını ve yenile butonunu gösterir
  3. Kullanıcı kartları bölümler içinde sürükleyerek sıralayabilir ve küçük / orta / büyük boyut seçebilir; düzen uygulama yeniden açıldığında korunur
  4. Kullanıcı bir öğeye tıkladığında detaylar yandan kayan ortak panelde açılır; Ctrl+K ile açılan komut paletinde arama yapıp kayıtlı eylemleri çalıştırabilir

**Plans**: TBD
**UI hint**: yes

### Phase 3: Bağlantı Altyapısı, Bildirim Hattı ve API Kartları

**Goal**: Kullanıcı herhangi bir servisi uygulama koduna dokunmadan (bildirimsel API tanımıyla) ekleyip panoda kart olarak görebilir, tüm bağlantıları tek yerden yönetebilir ve bildirimler kontrollü, gruplanmış şekilde gelir
**Depends on**: Phase 2
**Requirements**: CONN-01, CONN-02, CONN-03, CONN-04, CONN-05, CONN-06, CARD-01, CARD-02, CARD-03, CARD-04
**Success Criteria** (what must be TRUE):

  1. Kullanıcı bir API adresi girip (gerekirse anahtar ekleyip) test isteği gönderir, JSON ağacından tıklayarak alan seçer, yenileme aralığı ve görselleştirme tipini (değer, liste, değer + mini grafik) belirler ve panoda kendiliğinden güncellenen bir kart elde eder
  2. Kullanıcı hazır katalogdan (hava durumu, döviz / altın, kripto, haber, namaz vakitleri, deprem) tek tıkla kart ekler; katalog girdileri ve kullanıcı tanımları aynı bildirimsel tanım biçimindedir, yeni servis türü için çekirdek kod değişmez
  3. Kullanıcı Bağlantılar sayfasında her servisin durumunu (Bağlı / Yeniden giriş gerekli / Hata / Hız sınırı / Duraklatıldı), hesabını, izinlerini, son ve sonraki senkronizasyonunu görür; bağlantıyı test eder, duraklatır, kaldırır (kaldırınca verileri ve anahtarları silinir) ve tek tıkla tüm takibi duraklatıp devam ettirir
  4. Kullanıcı "dolar 45'i geçerse" gibi bir eşik koşulu tanımladığında değer eşiği geçince bildirim alır; yeni bağlanan bir kaynağın mevcut verileri sessizce alınır, yalnızca sonraki değişiklikler bildirilir
  5. Kullanıcı bildirim merkezinde geçmiş bildirimleri görür; servis başına bildirim ayarı, sessiz saatler ve gruplama ("5 yeni e-posta") uygulanır

**Plans**: TBD
**UI hint**: yes

### Phase 4: Google Bağlayıcısı ve Günün Özeti Ana Ekranı

**Goal**: Kullanıcı Google hesabını güvenle bağlar; mailleri ve takvimi Mornfold'da okur, önemli olaylarda bildirim alır ve uygulamayı açtığında AI olmadan da çalışan "Günün özeti" ana ekranını görür
**Depends on**: Phase 3
**Requirements**: GOOG-01, GOOG-02, GOOG-03, GOOG-04, GOOG-05, GOOG-06, GOOG-07
**Success Criteria** (what must be TRUE):

  1. Kullanıcı Google hesabını sistem tarayıcısı üzerinden PKCE ile bağlar ve isterse kendi Google Cloud istemci kimliğini kullanır; oturum düştüğünde tek tıkla yeniden bağlanabileceği bir kart görür
  2. Kullanıcı önemli / okunmamış mailleri bir kartta görür ve bir maili uygulama içinde güvenli şekilde (komut dosyası çalışmadan, uzak görseller varsayılan engelli) okur
  3. Kullanıcı bugünün ve yaklaşan günlerin takvim ajandasını görür ve bir sonraki toplantıya "Katıl" butonuyla girer
  4. Kullanıcı önemli bir mail geldiğinde ve bir toplantı yaklaştığında Windows bildirimi alır; bildirime tıklayınca ilgili mail veya etkinlik açılır
  5. Kullanıcı uygulamayı açtığında AI anahtarı olmadan da toplantıları, önemli mailleri, kartları ve dikkat isteyenleri gösteren "Günün özeti" ana ekranını görür

**Plans**: TBD
**UI hint**: yes

### Phase 5: AI Sabah Özeti

**Goal**: Kullanıcı her sabah, kendi seçtiği AI sağlayıcısıyla yazılmış, her maddesi kaynağına bağlı, hiçbir işlem yapamayan ve maliyeti / gönderilen verisi şeffaf Türkçe bir günün özeti okur
**Depends on**: Phase 4
**Requirements**: AI-01, AI-02, AI-03, AI-04, AI-05, AI-06, AI-07, AI-08, AI-10
**Success Criteria** (what must be TRUE):

  1. Kullanıcı Claude, Gemini veya OpenRouter'ı seçer, kendi API anahtarını girip test eder ve modeli seçer; belirlediği saatte ya da bilgisayarı ilk açtığında Türkçe AI özetini görür
  2. Özetteki her madde kaynağına (mail, etkinlik, kart) bağlıdır ve tıklanınca o öğe açılır; kullanıcı özeti yeniden oluşturabilir ve geçmiş özetlere bakabilir
  3. AI anahtarı yoksa veya sağlayıcı hata verirse kullanıcı yine şablon tabanlı bir özet görür; gizli komut içeren test maili / sayfası özeti ele geçiremez ve hiçbir işlem tetikleyemez (özet yalnızca düz metindir)
  4. Mail içeriği AI sağlayıcısına ilk kez gönderilmeden önce kullanıcıdan açık onay alınır; kullanıcı özet başına tahmini maliyeti, aylık harcamayı ve AI'ya gönderilen verileri görür, aylık harcama limiti koyabilir ve limit aşılınca AI çağrısı yapılmaz
  5. Kullanıcı "benim için önemli olanlar" listesi ve hazır şablonlarla özeti kişiselleştirir ve sonraki özet bu tercihlere göre şekillenir

**Plans**: TBD
**UI hint**: yes

### Phase 6: Aksiyon Hattı

**Goal**: Kullanıcı Mornfold'dan mail yanıtlar, mail düzenler ve takvime etkinlik ekler; dışarıya giden her işlem tam önizleme ve onaydan geçer, hiçbir şey kendiliğinden yapılmaz ve her işlem kayıt altındadır
**Depends on**: Phase 5
**Requirements**: ACT-01, ACT-02, ACT-03, ACT-04, ACT-05, ACT-06, AI-09
**Success Criteria** (what must be TRUE):

  1. Kullanıcı bir maili uygulama içinden yanıtlar veya tümünü yanıtlar; yanıt Gmail'de doğru konu zincirinde ve Türkçe karakterler bozulmadan görünür; gönder'e bastıktan sonra 8 saniye içinde gönderimi geri alabilir
  2. Kullanıcı bir maili arşivler, okundu işaretler ve yıldızlar; değişiklik Gmail'e ve karta yansır
  3. Kullanıcı takvime etkinlik ekler; davetli varsa gönderimden önce alıcılar, içerik ve saat dilimiyle tam önizleme görüp onaylar
  4. Özetteki aksiyon butonları ("Bu maili yanıtla", "Toplantıyı aç") yalnızca kullanıcı tıklayınca çalışır; zamanlanmış görev, bildirim tıklaması veya AI çıktısı hiçbir dış işlemi kendiliğinden başlatamaz
  5. Kullanıcı yapılan tüm işlemlerin kaydını (denetim günlüğü) görebilir

**Plans**: TBD
**UI hint**: yes

### Phase 7: Web Sitesi Değişiklik Takibi

**Goal**: Kullanıcı istediği bir web sayfasını izlemeye alır ve sayfa anlamlı şekilde değiştiğinde (ya da tanımladığı koşul gerçekleştiğinde) bildirim alır; gürültü ve hatalar sahte değişiklik olarak görünmez
**Depends on**: Phase 3
**Requirements**: WEB-01, WEB-02, WEB-03, WEB-04, WEB-05, WEB-06
**Success Criteria** (what must be TRUE):

  1. Kullanıcı bir URL ve isteğe bağlı CSS seçici girer, kaydetmeden önce "şimdi test et" ile neyin izleneceğini görür; kontrol aralığını (en az 5 dakika) seçer ve JavaScript ile yüklenen sayfalar için "tarayıcıda işle" seçeneğini açabilir
  2. Sayfa değiştiğinde kullanıcı bildirim alır ve kelime bazlı farkı (ne eklendi / silindi) ve değişiklik geçmişini görür
  3. Kullanıcı "metin görünürse", "sayı bir değerin altına düşerse" veya "% azalırsa" koşulu tanımlar ve yalnızca koşul gerçekleşince bildirim alır
  4. Kullanıcı tarih, sayaç gibi gürültüleri yok sayma kuralı ekler ve bu değişiklikler artık bildirim üretmez
  5. Site erişilemez, seçici bulunamaz veya bot koruması varsa kullanıcı sahte değişiklik değil, açık bir hata durumu görür

**Plans**: TBD
**UI hint**: yes

### Phase 8: MCP Eklentileri ve İlk Kurulum

**Goal**: Kullanıcı herhangi bir servisi MCP sunucusu olarak güvenle ekleyip panoya kart olarak taşıyabilir; yeni bir kullanıcı Mornfold'u ilk açılışta adım adım kendi hesaplarıyla kurabilir
**Depends on**: Phase 6
**Requirements**: MCP-01, MCP-02, MCP-03, MCP-04, MCP-05, MCP-06, MCP-07, ONBD-01, ONBD-02
**Success Criteria** (what must be TRUE):

  1. Kullanıcı yerel (komutla) veya uzak (HTTP) MCP sunucusu ekler ya da `mcpServers` JSON bloğunu yapıştırarak içe aktarır; yerel sunucuda çalıştırılacak tam komutu riskli kalıplar vurgulanmış hâlde görüp onaylar; Node.js / Python eksikse açık bir uyarı görür
  2. Kullanıcı her aracı ayrı ayrı açıp kapatır; sunucunun araç listesi onaydan sonra değişirse sunucu devre dışı kalır ve fark gösterilir
  3. Kullanıcı salt okunur bir MCP aracını veya kaynağını belirli aralıkla yenilenen bir pano kartına dönüştürür; MCP araçlarıyla yapılan her yazma işlemi önizleme + onaydan geçer
  4. MCP sunucuları konsol penceresi açmadan çalışır, çökünce yeniden başlar, uygulama kapanınca sonlanır; kullanıcı sunucu loglarını görebilir
  5. İlk açılışta kullanıcı AI anahtarı, Google bağlantısı (kendi Google Cloud projesi rehberiyle), başlangıç kartları, bildirim ve otomatik başlatma adımlarından geçer; her adımı atlayıp sonradan Ayarlar'dan tamamlayabilir

**Plans**: TBD
**UI hint**: yes

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Windows Kabuğu ve Platform Altyapısı | 0/19 | Planned | - |
| 2. Tasarım Sistemi ve Kart Kütüphanesi | 0/TBD | Not started | - |
| 3. Bağlantı Altyapısı, Bildirim Hattı ve API Kartları | 0/TBD | Not started | - |
| 4. Google Bağlayıcısı ve Günün Özeti Ana Ekranı | 0/TBD | Not started | - |
| 5. AI Sabah Özeti | 0/TBD | Not started | - |
| 6. Aksiyon Hattı | 0/TBD | Not started | - |
| 7. Web Sitesi Değişiklik Takibi | 0/TBD | Not started | - |
| 8. MCP Eklentileri ve İlk Kurulum | 0/TBD | Not started | - |

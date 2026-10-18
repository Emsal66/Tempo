# Requirements: Mornfold

**Defined:** 2026-10-18
**Core Value:** Dağınık servislerin hepsini tek, zarif bir ekranda toplayıp oradan yönetebilmek; her şey bozulsa bile zarif kabuk + servis bağlama altyapısı çalışmalı.

## v1 Requirements

Requirements for initial release. Each maps to roadmap phases.

### Kabuk ve Windows Altyapısı (SHELL)

- [ ] **SHELL-01**: Kullanıcı Mornfold'u Windows 11'e bir kurulum dosyasıyla (NSIS) kurabilir
- [ ] **SHELL-02**: Kullanıcı pencereyi kapattığında uygulama sistem tepsisinde çalışmaya devam eder; tepsi menüsünden Aç / Tümünü duraklat / Çık seçilebilir
- [ ] **SHELL-03**: Kullanıcı Windows açılışında uygulamanın tepside (küçültülmüş) otomatik başlamasını açıp kapatabilir
- [ ] **SHELL-04**: Uygulama ikinci kez başlatıldığında yeni kopya açılmaz, mevcut pencere öne gelir
- [ ] **SHELL-05**: Kullanıcı, Mornfold adı ve ikonuyla gelen, tıklanınca ilgili öğeye götüren Windows bildirimleri alır
- [ ] **SHELL-06**: Bilgisayar uykudan uyandığında kaçırılan kontroller bir kez, sırayla çalıştırılır; tepside "son kontrol: X dk önce" sağlık satırı görünür
- [ ] **SHELL-07**: Tüm API anahtarları ve OAuth jetonları yalnızca kullanıcının PC'sinde, Windows Kimlik Bilgisi Yöneticisi'ne bağlı şifreli kasada saklanır; arayüz anahtarları geri okuyamaz ve loglarda görünmez
- [ ] **SHELL-08**: Uygulama arayüzü Türkçe'dir (çok dilli altyapı ile)
- [ ] **SHELL-09**: Kullanıcı uygulamayı yeni sürüme uygulama içinden güncelleyebilir (imzalı güncelleme)

### Tasarım Sistemi ve Pano (DSGN)

- [ ] **DSGN-01**: Uygulama Apple / Linear tarzı görünür: Mica cam arka plan, özel başlık çubuğu, ince tipografi, yumuşak animasyonlar
- [ ] **DSGN-02**: Kullanıcı açık / koyu tema ve "şeffaflığı azalt" seçenekleri arasında geçiş yapabilir
- [ ] **DSGN-03**: Her kart yükleniyor / boş / hata / eski veri durumlarını, son güncelleme zamanını ve yenile butonunu gösterir
- [ ] **DSGN-04**: Kullanıcı kartları bölümler içinde sıralayabilir ve küçük / orta / büyük boyut seçebilir
- [ ] **DSGN-05**: Kullanıcı bir öğeye tıkladığında detaylar yandan kayan ortak bir panelde açılır
- [ ] **DSGN-06**: Kullanıcı Ctrl+K komut paletiyle arama yapabilir ve bağlayıcıların sunduğu eylemleri çalıştırabilir

### Bağlantı Altyapısı ve Bildirimler (CONN)

- [ ] **CONN-01**: Kullanıcı Bağlantılar sayfasında her servisin durumunu (Bağlı / Yeniden giriş gerekli / Hata / Hız sınırı / Duraklatıldı), hesabı, izinleri, son ve sonraki senkronizasyonu görebilir
- [ ] **CONN-02**: Kullanıcı bir bağlantıyı test edebilir, duraklatabilir ve kaldırabilir; kaldırınca o servisin verileri ve anahtarları silinir
- [ ] **CONN-03**: Kullanıcı tek tıkla tüm takip ve senkronizasyonu duraklatıp devam ettirebilir
- [ ] **CONN-04**: Yeni bir servis ilk bağlandığında mevcut veriler sessizce alınır (yüzlerce bildirim gelmez); sonraki değişiklikler bildirilir
- [ ] **CONN-05**: Kullanıcı bildirim merkezinde geçmiş bildirimleri görebilir; servis başına bildirim ayarı, sessiz saatler ve gruplama ("5 yeni e-posta") uygulanır
- [ ] **CONN-06**: Yeni servis türleri uygulamanın çekirdek koduna dokunmadan eklenebilir (bildirimsel API tanımı veya MCP sunucusu ile)

### API Kartları ve Katalog (CARD)

- [ ] **CARD-01**: Kullanıcı bir API adresi girip (gerekirse anahtar ekleyip) test isteği gönderebilir, gelen JSON ağacından tıklayarak gösterilecek alanı seçebilir ve bunu bir karta dönüştürebilir
- [ ] **CARD-02**: Kullanıcı kart için yenileme aralığını ve görselleştirme tipini (değer, liste, değer + mini grafik) seçebilir
- [ ] **CARD-03**: Kullanıcı hazır katalogdan tek tıkla kart ekleyebilir: hava durumu, döviz / altın, kripto, haber ve Türkiye'ye özel kaynaklar (ör. namaz vakitleri, deprem)
- [ ] **CARD-04**: Kullanıcı bir değer için eşik koşulu tanımlayıp (ör. "dolar 45'i geçerse") bildirim alabilir

### Google (GOOG)

- [ ] **GOOG-01**: Kullanıcı Google hesabını tarayıcı üzerinden güvenli şekilde (PKCE) bağlayabilir; isterse kendi Google Cloud istemci kimliğini kullanabilir
- [ ] **GOOG-02**: Kullanıcı önemli / okunmamış mailleri bir kartta görebilir
- [ ] **GOOG-03**: Kullanıcı bir maili uygulama içinde güvenli şekilde (komut dosyası çalışmadan, uzak görseller varsayılan engelli) okuyabilir
- [ ] **GOOG-04**: Kullanıcı bugünün ve yaklaşan günlerin takvim ajandasını görebilir; bir sonraki toplantıya "Katıl" butonuyla girebilir
- [ ] **GOOG-05**: Kullanıcı önemli bir mail geldiğinde ve toplantı yaklaştığında bildirim alır
- [ ] **GOOG-06**: Google oturumu düştüğünde kullanıcı tek tıkla yeniden bağlanabileceği bir kart görür
- [ ] **GOOG-07**: Kullanıcı uygulamayı açtığında AI olmadan da çalışan "Günün özeti" ana ekranını görür (toplantılar, önemli mailler, kartlar, dikkat isteyenler)

### AI Sabah Özeti (AI)

- [ ] **AI-01**: Kullanıcı AI sağlayıcısı olarak Claude, Gemini veya OpenRouter seçebilir, kendi API anahtarını girip test edebilir ve modeli seçebilir
- [ ] **AI-02**: Kullanıcı her sabah (belirlenen saatte ya da bilgisayarı ilk açtığında) Türkçe, AI ile yazılmış bir günün özeti görür
- [ ] **AI-03**: Özetteki her madde kaynağına (mail, etkinlik, kart) bağlıdır; tıklanınca o öğe açılır
- [ ] **AI-04**: AI özeti hiçbir işlem yapamaz, yalnızca düz metin üretir; mail / web içeriğine gizlenmiş komutlar özeti ele geçiremez
- [ ] **AI-05**: AI anahtarı yoksa veya sağlayıcı hata verirse kullanıcı yine şablon tabanlı bir özet görür
- [ ] **AI-06**: Kullanıcı özet başına tahmini maliyeti, aylık harcamayı ve AI'ya hangi verilerin gönderildiğini görebilir; aylık harcama limiti koyabilir
- [ ] **AI-07**: Mail içeriği AI sağlayıcısına ilk kez gönderilmeden önce kullanıcıdan açık onay alınır
- [ ] **AI-08**: Kullanıcı "benim için önemli olanlar" listesi ve hazır şablonlarla özeti kişiselleştirebilir
- [ ] **AI-09**: Özette önerilen aksiyon butonları (ör. "Bu maili yanıtla", "Toplantıyı aç") bulunur; yalnızca kullanıcı tıklayınca çalışır
- [ ] **AI-10**: Kullanıcı özeti yeniden oluşturabilir ve geçmiş özetlere bakabilir

### Aksiyonlar (ACT)

- [ ] **ACT-01**: Kullanıcı bir maili uygulama içinden yanıtlayabilir ve tümünü yanıtlayabilir (doğru konu zinciri, Türkçe karakterler bozulmadan)
- [ ] **ACT-02**: Kullanıcı gönder'e bastıktan sonra 8 saniye içinde gönderimi geri alabilir
- [ ] **ACT-03**: Kullanıcı bir maili arşivleyebilir, okundu işaretleyebilir ve yıldızlayabilir
- [ ] **ACT-04**: Kullanıcı uygulama içinden takvime etkinlik ekleyebilir (davetli varsa gönderimden önce onay istenir)
- [ ] **ACT-05**: Dışarıya giden veya geri alınamaz her işlem öncesinde kullanıcı tam önizlemeyi (alıcılar, içerik, saat) görüp onaylar; hiçbir işlem zamanlanmış görev, bildirim veya AI tarafından kendiliğinden yapılmaz
- [ ] **ACT-06**: Kullanıcı yapılan işlemlerin kaydını (denetim günlüğü) görebilir

### Web Sitesi Değişiklik Takibi (WEB)

- [ ] **WEB-01**: Kullanıcı bir URL ve isteğe bağlı CSS seçici ile site takibi ekleyebilir; kaydetmeden önce "şimdi test et" ile neyin izleneceğini görebilir
- [ ] **WEB-02**: Kullanıcı kontrol aralığını seçebilir (en az 5 dakika); JavaScript ile yüklenen sayfalar için "tarayıcıda işle" seçeneği vardır
- [ ] **WEB-03**: Kullanıcı sayfa değiştiğinde bildirim alır ve kelime bazlı farkı (ne eklendi / silindi) ve geçmişi görebilir
- [ ] **WEB-04**: Kullanıcı koşul tanımlayabilir: metin görünürse, sayı bir değerin altına düşerse veya % azalırsa bildir
- [ ] **WEB-05**: Kullanıcı tarih, sayaç gibi gürültüleri yok sayma kuralları ekleyebilir
- [ ] **WEB-06**: Site erişilemez, seçici bulunamaz veya bot koruması varsa kullanıcı sahte değişiklik değil, açık bir hata durumu görür

### MCP Eklentileri (MCP)

- [ ] **MCP-01**: Kullanıcı yerel (komutla çalışan) ve uzak (HTTP) MCP sunucusu ekleyebilir veya hazır `mcpServers` JSON bloğunu yapıştırarak içe aktarabilir
- [ ] **MCP-02**: Yerel bir sunucu eklenirken kullanıcı çalıştırılacak tam komutu görür ve onaylar; riskli kalıplar vurgulanır
- [ ] **MCP-03**: Kullanıcı her aracı ayrı ayrı açıp kapatabilir; sunucunun araç listesi onaydan sonra değişirse sunucu devre dışı kalır ve farkı gösterilir
- [ ] **MCP-04**: Kullanıcı salt okunur bir MCP aracını veya kaynağını belirli aralıkla çalışan bir pano kartına dönüştürebilir (MCP kartı)
- [ ] **MCP-05**: MCP araçlarıyla yapılan yazma işlemleri her zaman önizleme + onaydan geçer
- [ ] **MCP-06**: MCP sunucuları arka planda konsol penceresi açmadan çalışır, çökerse yeniden başlatılır, uygulama kapanınca sonlandırılır; kullanıcı sunucu loglarını görebilir
- [ ] **MCP-07**: Node.js / Python gerektiren bir sunucu için ön koşul eksikse kullanıcı açık bir uyarı görür

### İlk Kurulum (ONBD)

- [ ] **ONBD-01**: İlk açılışta kullanıcı adım adım bir kurulum sihirbazından geçer: AI anahtarı, Google bağlantısı (kendi Google Cloud projesini bağlama rehberiyle), başlangıç kartları, bildirim ve otomatik başlatma tercihleri
- [ ] **ONBD-02**: Sihirbazdaki her adım atlanabilir ve sonradan Ayarlar'dan tamamlanabilir

## v2 Requirements

Deferred to future release. Tracked but not in current roadmap.

### X / Twitter

- **XTW-01**: Kullanıcı seçtiği X hesapları yeni paylaşım yapınca bildirim alır (kendi X API anahtarıyla)
- **XTW-02**: Kullanıcı kelime / hashtag / konu takibi yapabilir
- **XTW-03**: Kullanıcı takip kaydetmeden önce maliyet tahmini görür, takip başına ve genel bütçe limiti koyabilir

### Dağıtım

- **DIST-01**: Kurulum dosyası kod imzalıdır (Microsoft Store veya OV sertifika) — kanal kararı Faz 1'de yazılı olarak alınır
- **DIST-02**: Google doğrulaması / CASA için karar verilir; uygulama 100+ kullanıcıya açılabilir
- **DIST-03**: Gizlilik politikası yayımlanır

### Gelişmiş

- **ADV-01**: Görsel öğe seçici (sayfada tıklayarak izlenecek alanı seçme)
- **ADV-02**: AI ile "sadece şu durumda bildir" filtresi ve değişiklik özetleri
- **ADV-03**: AI ile yanıt taslağı
- **ADV-04**: Akşam özeti / gün sonu kapanışı
- **ADV-05**: Birden fazla Google hesabı (veri modeli v1'den hazır)
- **ADV-06**: Yerel LLM (OpenAI uyumlu adres)
- **ADV-07**: Genel kısayol ile uygulamayı her yerden çağırma
- **ADV-08**: Türkçe doğal dil ile hızlı ekleme ("yarın 15:00 Ali ile toplantı")

## Out of Scope

| Feature | Reason |
|---------|--------|
| X/Twitter veri kazıma (scraping), Nitter | X kuralları yasaklıyor, yüksek tazminat riski; Nitter kapandı |
| Kendi X bildirimlerini (mention/DM) takip, X'e paylaşım yapma | Kullanıcı seçmedi |
| AI'nın kendiliğinden işlem yapması (araç kullanan AI ajanı) | Prompt injection riski; her işlem kullanıcı tıklamasıyla |
| Üçüncü parti kod eklentileri (JS/native), eklentinin kendi arayüzünü getirmesi | Güvenlik; eklentiler sadece veri sağlar, arayüz sabit kart kütüphanesinden |
| Serbest piksel tuval, YAML ile yapılandırma | Zarif ve basit deneyimi bozar |
| Gömülü web uygulamaları (Rambox tarzı) | Ürün odağı dışında, ağır |
| Gmail Pub/Sub push | Yerel uygulama için sunucu gerektirir; yoklama (polling) kullanılır |
| Captcha aşma, saniye bazlı kontrol aralığı | Yasal / etik ve yük sorunları |
| Kullanıcı oturumu kapalıyken çalışan Windows servisi | Gereksiz karmaşıklık |
| Bulut senkronizasyonu, ortak/ekip panoları | Her şey yerel kalacak; başkaları kendi PC'lerinde kullanacak |
| macOS / Linux / mobil sürüm, Windows 10 | Sadece Windows 11 hedefleniyor |
| Site uptime izleme, RSS, site istatistikleri | Kullanıcı yalnızca değişiklik takibini seçti |
| Outlook, Notion, Todoist yerleşik bağlayıcıları | Kullanıcı Google kullanıyor; MCP ile eklenebilir |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| SHELL-01 | Phase 1 | Pending |
| SHELL-02 | Phase 1 | Pending |
| SHELL-03 | Phase 1 | Pending |
| SHELL-04 | Phase 1 | Pending |
| SHELL-05 | Phase 1 | Pending |
| SHELL-06 | Phase 1 | Pending |
| SHELL-07 | Phase 1 | Pending |
| SHELL-08 | Phase 1 | Pending |
| SHELL-09 | Phase 1 | Pending |
| DSGN-01 | Phase 2 | Pending |
| DSGN-02 | Phase 2 | Pending |
| DSGN-03 | Phase 2 | Pending |
| DSGN-04 | Phase 2 | Pending |
| DSGN-05 | Phase 2 | Pending |
| DSGN-06 | Phase 2 | Pending |
| CONN-01 | Phase 3 | Pending |
| CONN-02 | Phase 3 | Pending |
| CONN-03 | Phase 3 | Pending |
| CONN-04 | Phase 3 | Pending |
| CONN-05 | Phase 3 | Pending |
| CONN-06 | Phase 3 | Pending |
| CARD-01 | Phase 3 | Pending |
| CARD-02 | Phase 3 | Pending |
| CARD-03 | Phase 3 | Pending |
| CARD-04 | Phase 3 | Pending |
| GOOG-01 | Phase 4 | Pending |
| GOOG-02 | Phase 4 | Pending |
| GOOG-03 | Phase 4 | Pending |
| GOOG-04 | Phase 4 | Pending |
| GOOG-05 | Phase 4 | Pending |
| GOOG-06 | Phase 4 | Pending |
| GOOG-07 | Phase 4 | Pending |
| AI-01 | Phase 5 | Pending |
| AI-02 | Phase 5 | Pending |
| AI-03 | Phase 5 | Pending |
| AI-04 | Phase 5 | Pending |
| AI-05 | Phase 5 | Pending |
| AI-06 | Phase 5 | Pending |
| AI-07 | Phase 5 | Pending |
| AI-08 | Phase 5 | Pending |
| AI-09 | Phase 6 | Pending |
| AI-10 | Phase 5 | Pending |
| ACT-01 | Phase 6 | Pending |
| ACT-02 | Phase 6 | Pending |
| ACT-03 | Phase 6 | Pending |
| ACT-04 | Phase 6 | Pending |
| ACT-05 | Phase 6 | Pending |
| ACT-06 | Phase 6 | Pending |
| WEB-01 | Phase 7 | Pending |
| WEB-02 | Phase 7 | Pending |
| WEB-03 | Phase 7 | Pending |
| WEB-04 | Phase 7 | Pending |
| WEB-05 | Phase 7 | Pending |
| WEB-06 | Phase 7 | Pending |
| MCP-01 | Phase 8 | Pending |
| MCP-02 | Phase 8 | Pending |
| MCP-03 | Phase 8 | Pending |
| MCP-04 | Phase 8 | Pending |
| MCP-05 | Phase 8 | Pending |
| MCP-06 | Phase 8 | Pending |
| MCP-07 | Phase 8 | Pending |
| ONBD-01 | Phase 8 | Pending |
| ONBD-02 | Phase 8 | Pending |

**Coverage:**
- v1 requirements: 63 total
- Mapped to phases: 63
- Unmapped: 0 ✓

---
*Requirements defined: 2026-10-18*
*Last updated: 2026-10-18 after roadmap creation*

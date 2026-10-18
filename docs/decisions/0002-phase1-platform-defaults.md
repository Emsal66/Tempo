# ADR 0002 — Faz 1 platform varsayılanları

## Durum

Kabul edildi — 2026-10-18

İlgili kararlar: D-08, D-09, D-10, D-12, D-13, D-17, D-18 (`01-CONTEXT.md`) ve araştırmadaki açık
sorular / varsayımlar (`01-RESEARCH.md` → Open Questions, Assumptions Log A8, A9).

## Bağlam

Faz 1 araştırması bazı soruları açık bıraktı ve bazı varsayımlara dayandı. Planların tutarlı
uygulanabilmesi için her birine bir varsayılan seçilmesi gerekiyor. Bu kayıt o varsayılanları,
gerekçelerini ve hangi koşulda yeniden değerlendirileceklerini tek yerde toplar.

## Karar

### 1. "Yarına kadar" duraklatma (D-13, A9)

- "Tümünü duraklat → Yarına kadar" = **bir sonraki yerel gece yarısı**; sistem saat dilimine göre
  `jiff` ile hesaplanır (DST güvenli).
- Arayüzde açıkça **"devam: yarın 00:00"** olarak gösterilir (tepsi menüsü ve ipucu).
- Sahip bir sabah saatini (ör. 08:00) tercih ederse değiştirmek önemsizdir: tek bir hesaplama
  fonksiyonu.

### 2. Bildirime tıklayınca etkinleştirme (D-08)

- **Katman 1 (varsayılan):** toast XML'inde `activationType="protocol"`; gövde ve butonlar
  `mornfold://` bağlantılarıdır (ör. `mornfold://open/settings`). Bu yol COM sunucusu gerektirmeden
  hem uygulama çalışırken (sıcak) hem kapalıyken (soğuk) çalışır: Windows URL şemasını kayıtlı
  uygulamaya iletir, tek örnek eklentisi mevcut pencereyi öne getirir.
- **İsteğe bağlı:** Bildirim Merkezi'ndeki bildirimler kalıcı olmazsa (Windows kısayolda
  `ToastActivatorCLSID` bekliyorsa) NSIS kısayoluna yalnızca bir saplama (stub)
  `ToastActivatorCLSID` eklenir.
- **Katman 2 (yedek):** tam bir COM etkinleştirici, yalnızca smoke **S-6(b)** (uygulama kapalıyken
  Bildirim Merkezi'nden tıklama) başarısız olursa uygulanır.
- `mornfold://` üzerinden gelen eylemler doğrulanır: durum değiştiren eylemler (ör.
  `mornfold://act/mute?n=…`) yalnızca uygulamanın kendi gönderdiği, bilinen bildirim kimlikleri için
  kabul edilir; dışarıdan yazılan bağlantılar reddedilir.

### 3. Bildirim sesi (D-09, D-10)

- **Strateji A (varsayılan):** toast XML'inde `<audio src="file:///…/mornfold-ping.wav">`; ses
  Windows bildirim sistemiyle birlikte çalar ve Odak / Rahatsız etmeyin ayarlarına Windows uyar.
- **Strateji B:** toast sessiz gönderilir; uygulama WAV dosyasını `PlaySoundW` ile kendisi çalar —
  yalnızca ses açıkken, bildirimler etkinken ve `SHQueryUserNotificationState`
  `QUNS_ACCEPTS_NOTIFICATIONS` döndürdüğünde.
- Smoke **S-7** hangi stratejinin gerçek Win11 PC'de çalıştığını belirleyene kadar varsayılan A'dır.
  Ayarlar'da strateji seçilebilir ("Windows bildirimiyle çal" / "Uygulama kendisi çalsın").
- Ses tamamen veya kaynak bazında kapatılabilir. **Sessiz saatler** Phase 3 (bildirim motoru)
  kapsamındadır; Faz 1 yalnızca ses aç/kapa ayarını ve altyapısını sağlar.

### 4. Sesin kaynağı (D-09)

- `src-tauri/resources/sounds/mornfold-ping.wav`, `scripts/gen-ping.py` tarafından üretilen
  **özgün bir sentezdir** (sönümlenen sinüs kısmi tonları; deterministik, rastgelelik yok).
- Ses projeye aittir. Discord'un bildirim sesinin *karakterinden* (kısa, yumuşak, tanınabilir bir
  "ping") esinlenmiştir, ancak onun sesinden türetilmemiştir ve hiçbir üçüncü taraf sesini
  kopyalamaz.

### 5. Tepsi rozeti (D-12, A8)

- Okunmamış bildirim için simgede **küçük bir nokta**; **tam sayı ipucunda** ("3 okunmamış").
  32×32 tepsi simgesinde okunabilir bir sayı çizmek mümkün değildir.
- Duraklatıldığında simge **soluk** (yaklaşık %40 opaklık).
- Sağlık durumu **Hata** olduğunda simgede **kırmızı nokta**.
- Açık ve koyu görev çubuğu için ayrı simge setleri (`src-tauri/icons/tray/light`, `…/dark`).
- Bu görünüm smoke S-3'te sahibe sorulur.

### 6. Zamanlayıcı test işleri (Claude'un takdiri)

| İş | Sıklık | Duraklatılabilir | Amaç |
|---|---|---|---|
| `heartbeat` | 5 dakikada bir | Hayır | "son kontrol" zamanını besler |
| `demo_check` | 15 dakikada bir | Evet | Duraklatma ve telafi davranışını gösterir |
| `update_check` | 6 saatte bir + açılışta | Hayır | Güncelleme denetimi (D-02) |

Kaçırılan çalışmalar (uyku/kapalı) uyanınca ve açılışta **bir kez** telafi edilir ("telafi"
işaretiyle); art arda çalıştırma patlaması yapılmaz.

### 7. SQL erişimi

- **Çalışma zamanında denetlenen statik SQL:** `sqlx::query` / `sqlx::query_as` + bind
  parametreleri; her sorgu testlerle kapsanır. `query!` makroları ve `.sqlx/` meta verisi
  kullanılmaz.
- Gerekçe: makroların asıl maliyeti operasyoneldir — bir `DATABASE_URL` ve her yürütücünün ve her CI
  işinin taze tutması gereken bir `cargo sqlx prepare` adımı (`.sqlx/` çıktısı) gerektirirler.
  Faz 1'in küçük sorgu kümesi testlerle tamamen çalıştırıldığından bu takas Faz 1 için kabul
  edilebilir.
- **Yeniden değerlendirme tetikleyicisi (somut):** bu karar Phase 3 bağlayıcı (connector)
  tablolarını eklemeden önce yeniden değerlendirilir. Sorgu sayısı kabaca 30 ifadeyi aşarsa veya bir
  şema değişikliği testlerin yakalamadığı bir sorguyu çalışma zamanında bozarsa,
  `query!` / `query_as!` + commit edilmiş `.sqlx/` meta verisine geçilir.

### 8. Anahtar testleri (D-17)

- `mornfold-core` içinde bir **`ProviderTester`** bulunur (reqwest 0.13, rustls). Bir anahtarı
  kaydetmeden önce sağlayıcısının hafif kimlik doğrulama uç noktasına karşı dener:

  | Sağlayıcı | İstek |
  |---|---|
  | Anthropic | `GET https://api.anthropic.com/v1/models` |
  | Gemini | `GET https://generativelanguage.googleapis.com/v1beta/models` |
  | OpenRouter | `GET https://openrouter.ai/api/v1/key` |
  | OpenAI | `GET https://api.openai.com/v1/models` |
  | GitHub | `GET https://api.github.com/user` |
  | Slack | `POST https://slack.com/api/auth.test` |

- Sağlayıcı anahtarın önekinden (`detect_provider`) tespit edilir. Sağlayıcısı tanınmayan anahtarlar
  testi atlar (`NotTestable`) ve doğrudan kaydedilir ("Bu anahtarın sağlayıcısı tanınmadı; test
  atlandı.").
- Geçersiz bir anahtar Türkçe **"geçersiz"** uyarısını gösterir ve yalnızca açıkça **"Yine de
  kaydet"** seçilirse kaydedilir.
- Kayıtlı bir anahtar, değeri yeniden yazmadan tekrar test edilebilir (`secrets_test_saved`):
  çözme işlemi çekirdekte yapılır, arayüze yalnızca sonuç döner. Tam değer arayüze asla dönmez
  (D-15).
- Tüm ağ çağrıları Rust çekirdeğinde kalır (CLAUDE.md "What NOT to Use"); anahtarlar yalnızca
  yukarıdaki sabit HTTPS ana bilgisayarlarına gider, yönlendirmeler (redirect) izlenmez.
- Phase 5, Claude / Gemini / OpenRouter alanlarını aynı kasaya ve aynı test edicilere bağlar.

### 9. Kabul edilen risk

- DPAPI / Kimlik Bilgisi Yöneticisi, diğer Windows kullanıcılarına ve çevrimdışı disk hırsızlığına
  karşı korur; **aynı kullanıcı olarak çalışan kötü amaçlı yazılıma karşı korumaz**. Bu, tek
  kullanıcılı yerel bir masaüstü uygulaması için kabul edilen bir risktir.

## Sonuçlar

- Her açık sorunun planlarda uygulanabilir bir varsayılanı vardır; yürütücüler aynı varsayımla
  çalışır.
- Windows'a özgü belirsizlikler (bildirim etkinleştirme, ses stratejisi, rozet görünümü) smoke
  testlerine (S-3, S-6, S-7) bağlanmıştır; sonuçlar bu varsayılanları doğrular veya yedek yola
  geçirir.
- SQL kararı geçicidir ve somut bir yeniden değerlendirme tetikleyicisi vardır.

## Gözden geçirme

- Smoke S-6(b) başarısız → Katman 2 COM etkinleştirici.
- Smoke S-7 sonucu → varsayılan ses stratejisi A veya B olarak sabitlenir.
- Smoke S-3'te sahip rozet görünümünü beğenmezse → rozet tasarımı değişir.
- Phase 3 öncesi → SQL erişim kararı (madde 7).
- Sahip "Yarına kadar" için sabah saati isterse → madde 1.

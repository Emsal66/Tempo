# Mornfold

## What This Is

Mornfold, Windows'ta çalışan, Apple / Linear zarafetinde tasarlanmış kişisel bir komuta merkezidir. Kullanıcının hesaplarını, API'lerini, MCP sunucularını, ücretsiz ve ücretli üyeliklerini tek bir uygulamada birleştirir; sabah AI ile yazılmış bir "günün özeti" sunar ve gün boyu arka planda takip edip bildirim gönderir. İlk kullanıcı projenin sahibi; ileride başkaları da indirip kendi PC'lerinde, kendi hesaplarıyla kullanacak.

İsim: *morn* (sabah) + *fold* (sürünün toplandığı korunaklı ağıl) — "sabahın toplandığı yer".

## Core Value

Dağınık servislerin hepsini tek, zarif bir ekranda toplayıp oradan yönetebilmek: yeni bir servis eklemek kolay olmalı (eklenti altyapısı) ve uygulama açıldığında "hayran bırakan" bir deneyim sunmalı. Her şey bozulsa bile **zarif kabuk + servis bağlama altyapısı** çalışmalı.

## Requirements

### Validated

(None yet — ship to validate)

### Active

**Kabuk ve tasarım**
- [ ] Windows masaüstü uygulaması (kendi penceresi, kurulum dosyası)
- [ ] Apple / Linear tarzı arayüz: sade, bol boşluk, ince tipografi, yumuşak animasyonlar, cam (glass) efektleri
- [ ] Açılış ekranı "Günün özeti": bugünkü toplantılar, bekleyen işler, önemli mailler, dikkat isteyenler
- [ ] Sistem tepsisinde arka planda çalışma + Windows bildirimleri

**Eklenti / bağlantı altyapısı**
- [ ] Karma eklenti modeli: popüler servisler yerleşik bağlayıcı, diğerleri MCP sunucusu olarak eklenebilir (uygulama MCP istemcisi)
- [ ] Yeni servis eklemek uygulama koduna dokunmadan mümkün olmalı (MCP veya API tanımı ile)

**Google (ilk yerleşik bağlayıcı)**
- [ ] Gmail okuma ve uygulama içinden yanıtlama
- [ ] Google Takvim görüntüleme ve uygulama içinden etkinlik ekleme

**AI özet**
- [ ] AI ile yazılmış sabah özeti (düz metin, "bugün şunlar önemli")
- [ ] AI sağlayıcısı seçilebilir: Claude (Anthropic API), Gemini, OpenRouter — kullanıcı kendi API anahtarını girer

**Kapsamlı takip**
- [ ] Web sitesi değişiklik takibi: bir sayfanın içeriği değişince bildirim (fiyat düştü, yeni ilan, duyuru)
- [ ] Ücretsiz API ekleme — hazır katalog (popüler ücretsiz API'ler tek tıkla) + özel API tanımlama (URL gir, gösterilecek alanı seç, karta dönüşsün)

**Gizlilik**
- [ ] Tüm veriler ve API anahtarları yalnızca kullanıcının PC'sinde; anahtarlar Windows'un güvenli kasasında (Credential Manager / DPAPI); bulut sunucu yok

### Out of Scope

- Bulut senkronizasyonu / sunucu tarafı — kullanıcı her şeyin yerel kalmasını istedi
- Ortak/ekip kullanımı (paylaşımlı paneller) — "başkaları" kendi PC'lerinde ayrı ayrı kullanacak; paylaşımlı veri sunucu gerektirir
- macOS / Linux sürümü ve Windows 10 — v1 yalnızca Windows 11
- Twitter/X takibi v1'de yok (v2) — okuma başına ücretli (~$0,005/gönderi), kazıma yasak; araştırma sonrası ertelendi
- Kod imzası, Google doğrulaması, gizlilik politikası (dağıtım sertleştirme) — v2; kanal kararı Faz 1'de
- Mobil uygulama — asıl kullanım PC'de
- Twitter/X'e uygulama içinden paylaşım yapma — kullanıcı sadece izleme istedi
- Kendi X bildirimlerini (mention/DM) takip — kullanıcı seçmedi
- Site uptime izleme, RSS akışı, site istatistikleri — kullanıcı yalnızca değişiklik takibini seçti
- Microsoft (Outlook), Notion/Todoist bağlayıcıları — kullanıcı Google kullanıyor; MCP üzerinden sonradan eklenebilir

## Context

- Kullanıcı Türkçe konuşuyor; arayüz dili Türkçe olmalı (ileride başka kullanıcılar için çok dilli yapı düşünülebilir)
- Kullanıcının mevcut hesapları: Google (Gmail + Takvim), Claude, Gemini; ayrıca geliştirici araçları (GitHub vb.) ve finans/abonelikler — bunlar MCP/eklenti yoluyla sonraki adımlarda bağlanabilir
- İlk sürümde tek kullanıcı (proje sahibi); sonra başkalarına dağıtılacak → kurulum dosyası, ilk kurulum sihirbazı ve herkesin kendi API anahtarlarını/OAuth girişini yapması gerekecek
- Geliştirme bulut ortamında (Linux) yapılıyor; Windows'a özgü davranışlar (tepsi, bildirim, Credential Manager, kurulum) CI üzerinde Windows derlemesiyle doğrulanmalı
- Twitter/X API ücretli ve kısıtlı; X takibi için uygun yol araştırmada belirlenmeli (resmi API katmanları, alternatifler, maliyet)
- Google OAuth: Gmail kapsamları "restricted" sınıfında; başkalarına dağıtımda Google doğrulama süreci gerekebilir

## Constraints

- **Platform**: Yalnızca Windows 11 masaüstü — kullanıcının işletim sistemi; Mica cam efekti Windows 11'e özgü
- **Gizlilik**: Veriler ve anahtarlar yalnızca yerel — kullanıcı tercihi; sunucu yok
- **Tasarım**: Apple / Linear estetiği, "hayran bırakan zarafet" — ürünün ana farkı
- **Genişletilebilirlik**: Yeni servis = yeni eklenti/MCP sunucusu, çekirdek koda dokunmadan
- **Maliyet**: AI ve X gibi ücretli API'ler kullanıcının kendi anahtarıyla; uygulama ücret aracı değil

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| İsim: Mornfold | Sabah özeti odaklı, özgün; alan adları ve aramada boş görünüyor (resmi marka sorgusu yapılmadı) | — Pending |
| Karma eklenti modeli (yerleşik + MCP istemcisi) | Popüler servisler hızlı ve güvenilir; geri kalanı kod değişmeden eklenebilir | — Pending |
| AI sağlayıcısı seçilebilir (Claude / Gemini / OpenRouter) | Kullanıcının birden fazla AI hesabı var; tek sağlayıcıya bağımlılık yok | — Pending |
| Tüm takip özellikleri v1'de (X hariç) | Kullanıcı ilk sürümün ancak hepsiyle anlamlı olduğunu belirtti; X maliyet nedeniyle v2'ye ertelendi | — Pending |
| Tauri 2 + Rust çekirdek + React arayüz | Küçük kurulum, düşük bellek, yerleşik Mica, resmi Rust MCP SDK; çekirdek Linux'ta test edilebilir | — Pending |
| Yatay katmanlar yol haritası (8 faz) | Kullanıcı tercihi | — Pending |
| AI özeti araç kullanamaz, her işlem kullanıcı onaylı | Prompt injection riski | — Pending |
| Yalnızca yerel veri, Windows güvenli kasası | Kullanıcının gizlilik tercihi | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd:complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-10-18 after research and roadmap*

# 📜 Değişim Günlüğü (Project Changelog)

`dirbrute` projesinde yapılan tüm teknik geliştirmeler ve hata düzeltmeleri bu dosyada doğrusal ve profesyonel bir kronolojiyle kayıt altına alınmaktadır.

## 🚀 [v1.1.0] - Stable (Current) - 2026-04-03

### ✨ Yeni Özellikler
- **Deep Stealth Mode**: Cloudflare ve AWS Shield gibi gelişmiş WAF sistemlerini atlatmak için asenkron *Contextual Blending* ve otonom *Smart Cooldown* motoru entegre edildi.
- **Rotating Proxy Engine**: Tekli proxy yerine virgülle ayrılmış genişletilebilir proxy havuzu desteği getirildi. `Arc<Vec<Client>>` ile thread-safe IP rotasyonu sağlandı.
- **Report Export**: Bulguların temiz JSON ve CSV formatlarında dışa aktarımı için `--output` (`-o`) ve `--format` (`-f`) parametreleri eklendi.

### 🛠️ İyileştirmeler & Hata Düzeltmeleri
- **Concurrency Management**: Yüksek hacimli (10k+ payload) taramalarda SSE veri kanalının ("Deadlock") tıkanması sorunu asenkron tampon (buffer) artırımıyla çözüldü.
- **Supply Chain Security**: `cargo-deny` yapılandırması tamamlanarak bağımlılık zafiyet denetimleri zorunlu hale getirildi.

## 🧱 [v1.0.0] - Stable (Legacy) - 2026-04-03

### ✨ Yeni Özellikler
- **Enhanced Fingerprinting**: Laravel, Django, GraphQL ve CI/CD (Artifactory, Jenkins) servisleri için özel imza algoritmaları eklendi.
- **Docker Support**: Çok aşamalı (multi-stage) derleme yapan ultra-compact `Dockerfile` yayınlandı.
- **Wordlist Intelligence**: Statik örüntü listesi 50+ yeni kritik dosya yoluyla zenginleştirildi.
- **UI/UX Polish**: Glassmorphism arayüzündeki SSE senkronizasyon hataları ve görsel titremeler giderildi.

## 🧱 [v0.7.0] - Beta - 2026-04-03

### ✨ Yeni Özellikler
- **Heuristic 404 Filter**: Dinamik kalibrasyon motoruyla "Soft-404" (sahte 200) dönen sistemler için baseline analizi özelliği getirildi.
- **Tech Badge Engine**: Bulunan sonuçların yanında altyapı teknolojilerini (PHP, Docker, Node.js) gösteren görsel rozet sistemi kuruldu.

## 🧱 [v0.1.0] - İlk Sürüm - 2026-04-03
- Projenin Rust ile ilk asenkron sürümü yayınlandı.
- Paralel istek destekli çekirdek keşif motoru ve temel CLI yapısı kuruldu.

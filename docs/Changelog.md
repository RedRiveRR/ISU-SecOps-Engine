# Değişim Günlüğü (Changelog)

Directory Bruteforcer projesindeki tüm önemli değişiklikler bu dosyada kayıt altına alınacaktır.

## [v1.1.0] - 2026-04-03
### Eklendi
- **Deep Stealth Mode**: Güçlü WAF'ları (Cloudflare vb.) atlatabilmek için otonom *Contextual Blending* (zararsız araya kaynayan decoy istekler) ve hata durumunda 60 saniyelik *Cooldown* (soğuma) refleksleri motor seviyesine entegre edildi.
- **Rotating Proxy Pool**: Sisteme tek bir proxy sunucusu yerine virgülle ayrılmış yüzlerce çoklu proxy tanımlanabilmesi sağlandı. İşçiler (workers) her asenkron istekte havuzdan IP rotasyonu yaparak kusursuz anonimlik sağlar.
- **Düzeltme**: Yüksek hacimli taramalarda (10.000+ payload) arayüze veri sağlayan canlı SSE kanalının tıkanması ve taramayı askıda bırakması sorunu (Deadlock) çözüldü (`mpsc::channel` buffer artırıldı).
- **Veri Dışa Aktarma (Export)**: Tarama sonuçlarını JSON ve CSV formatında kaydetme özelliği eklendi (`--output` / `-o`).

## [v1.0.0] - 2026-04-03
### Eklendi
- **Genişletilmiş Parmak İzi**: Laravel, Django, GraphQL, Swagger ve CI/CD (Jenkins, GitLab) tespiti eklendi.
- **Docker Desteği**: Projenin konteynerize edilip hızlıca konuşlandırılması için çok aşamalı `Dockerfile` eklendi.
- **Daha Akıllı Tarama**: Statik örüntü listesine 50+ yeni kritik dosya yolu (api/health, .htaccess, artisan vb.) eklendi.
- **Gelişmiş Web UI**: Sonuçlar ve loglar arasındaki SSE senkronizasyon hataları giderildi.
- **Metadata Güncellemesi**: `Cargo.toml` üzerinden resmi proje açıklaması ve anahtar kelimeler eklendi.

## [v0.7.0] - 2026-04-03
### Eklendi
- **Soft-404 Heuristic Filter**: 
    - "Dinamik Kalibrasyon" motoru eklendi. Tarama öncesi otomatik olarak `/DirBrute-Heuristic-404-Test` yoluna istek atılarak sunucunun hata sayfası boyutu belirlenir.
    - Sahte 200 dönen sistemler için baseline karşılaştırmalı akıllı filtreleme getirildi.
- **Teknoloji Parmak İzi (Technology Fingerprinting)**:
    - Bulunan yollar üzerinden (WordPress, Node.js, Docker, PHP vb.) altyapı tespiti yapan motor entegre edildi.
    - Web UI üzerinde mor animasyonlu "Tech Badge" rozetleri eklendi.
- **Restorasyon ve Verimlilik**: Web UI v0.7.0 seviyesine yükseltildi, eksik tüm bileşenler geri yüklendi.

## [v0.6.0] - 2026-04-03
### Eklendi
- **Yeniden Yinelemeli (Recursive) Keşif**: 
    - Bulunan her dizinin altına otomatik olarak dalan derinlikli tarama motoru eklendi.
    - `--depth` (Web API: `depth`) parametresi ile tarama derinliği kontrol edilebilir hale getirildi.
- **Akıllı İçerik Analizi (Smart Content Analysis)**:
    - HTML `<title>` ve `Content-Length` bilgileri rapora eklendi.
- **Otomatik WAF Tespiti**:
    - `403/429` hataları üzerinden WAF sezgileme ve kullanıcıyı uyarma sistemi kuruldu.

## [v0.5.0] - 2026-04-03
### Eklendi
- **Dahili HTML Crawler**:
    - `nipper` kütüphanesi kullanılarak yüksek performanslı asenkron HTML ayrıştırma motoru entegre edildi.
    - Tarama sırasında bulunan tüm dahili bağlantıları otomatik olarak kuyruğa ekleyen dinamik kanal yapısı (`mpsc`) kuruldu.
    - Tekrarlı taramaları önlemek için `visited` cache mekanizması (HashSet) eklendi.
- **Web UI Geliştirmesi**:
    - Keşfedilen bağlantıları anlık olarak izlemek için "Crawler" sekmesi eklendi.
    - Tarama yapılandırmasına "Enable HTML Crawler" seçeneği eklendi.
- **CLI Güncellemesi**:
    - Crawler modunu aktifleştirmek için `--crawler` (`-C`) bayrağı eklendi.
- Proje sürümü `v0.5.0` olarak yükseltildi.

## [v0.4.0] - 2026-04-03
### Eklendi
- **Görünüm Ayrıştırma (UI/UX)**:
    - Web UI üzerinde "Bulunan Sonuçlar" ve "Canlı Loglar" sekmeleri eklendi.
    - Canlı loglar için 500 satırlık kayar pencere (rolling window) optimizasyonu yapıldı.
- **CLI Geliştirmesi**:
    - `--show-logs` (`-l`) parametresi eklendi. Aktifleştirildiğinde tüm denemeler anlık olarak terminale yazdırılır.
- Proje sürümü `v0.4.0` olarak yükseltildi.

## [v0.3.0] - 2026-04-03
### Eklendi
- **Akıllı Mod (Smart Mode)**:
    - **Statik Örüntülü Wordlist**: Dosya seçilmediğinde 100+ yüksek olasılıklı dosya/dizin yolunu otomatik olarak dener.
    - **Adaptif İş Parçacığı (Adaptive Threading)**: Hedef sunucunun yanıt hızına ve hata kodlarına göre eşzamanlı istek sayısını dinamik olarak ayarlar.
- **Web UI Güncellemesi**:
    - "Smart Mode" için yeni kontrol butonları ve görsel toggle'lar eklendi.
    - Gerçek zamanlı tarama hızı (Concurrency) göstergesi eklendi.
- **CLI Güncellemesi**: `--auto-wordlist` ve `--auto-threads` bayrakları eklendi.
- Proje sürümü `v0.3.0` olarak yükseltildi.

## [v0.2.4] - 2026-04-03
### Değiştirildi
- Dokümantasyon dosya isimleri tekrar İngilizceye çevrildi (`Architecture.md`, `Testing.md`, `Changelog.md`).
- Dokümantasyon içerikleri Türkçe olarak muhafaza edildi.
- Proje versiyonu `v0.2.4` olarak güncellendi.

## [v0.2.3] - 2026-04-03
### Eklendi
- Dokümantasyonun tamamen Türkçeleştirilmesi.
- Proje versiyonu `v0.2.3` olarak güncellendi.
### Planlanan
- Bundan sonraki tüm güncellemelerin Türkçe dokümantasyonla sunulması.

## [v0.2.2] - 2026-04-03
### Eklendi
- `/docs` dizini altına çekirdek dokümantasyon dosyaları eklendi.
- Proje versiyonu `v0.2.2` olarak güncellendi.

## [v0.2.1] - 2026-04-03
### Değiştirildi
- URL Yönetimi İyileştirildi: Protokol belirtilmediğinde otomatik olarak `https://` ekleme mantığı getirildi.
- Web UI'daki `background-clip` CSS uyarıları giderildi.
### Düzeltildi
- `http://` içermeyen alan adlarının tanınmaması sorunu giderildi.

## [v0.2.0] - 2026-04-03
### Eklendi
- Entegre Web Arayüzü: Modern, karanlık mod destekli cam efektli (glassmorphism) panel.
- `web` alt komutu ile gömülü sunucuyu başlatma desteği.
- Gerçek zamanlı tarama güncellemeleri için Server-Sent Events (SSE) altyapısı.
### Değiştirildi
- Çekirdek tarama motoru olay odaklı yapıya (`mpsc` kanalları) dönüştürüldü.
- CLI ve Web arayüzleri için çıktı yapısı birleştirildi.

## [v0.1.0] - 2026-04-03
### Eklendi
- `secops` aracının ilk sürümü yayınlandı.
- Yüksek performanslı, paralel istek destekli dizin keşif motoru.
- CLI üzerinden başlık (header), çerez (cookie) ve thread sayısı özelleştirme desteği.
- Bulunan yollar için ağaç yapısında terminal çıktısı.

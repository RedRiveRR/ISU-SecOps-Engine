# Değişim Günlüğü (Changelog)

ISU SecOps Engine projesindeki tüm önemli değişiklikler bu dosyada kayıt altına alınacaktır.

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

# 🚀 dirbrute v1.1.0 — High-Performance Stealth Discovery Engine

<div align="center">
  <img src="https://img.shields.io/badge/Rust-1.75%2B-orange.svg?style=for-the-badge&logo=rust" />
  <img src="https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20MacOS-lightgrey?style=for-the-badge" />
  <img src="https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Status-Stable-green.svg?style=for-the-badge" />
</div>

---

**dirbrute**, modern web güvenlik testleri için tasarlanmış, Rust tabanlı, ultra hızlı bir dizin ve dosya keşif motorudur. Geleneksel tarayıcıların ötesine geçerek **Deep Stealth** ve **Rotating Proxy** teknolojileriyle en katı firewall (WAF) mekanizmalarını dahi aşmanıza olanak tanır.

---

## 💎 Neden dirbrute?

Diğer araçların aksine `dirbrute`, sadece bir istek atıcı değil; bir **otonom keşif ekosistemidir**.

- **🥷 Deep Stealth Mode**: Cloudflare, Akamai ve AWS Shield gibi sistemlerin radarından kaçmak için akıllı "Contextual Blending" (aldatıcı trafik) ve dinamik soğuma (Cooldown) kullanır.
- **🔄 Rotating Proxy Pool**: Her bir istek için farklı bir IP mimarisi üzerinden çıkış yaparak kaynağınızı tamamen perdelere boğar.
- **🛰️ HTML Crawler & nipper**: Sayfa içerisindeki tüm linkleri anlık olarak ayrıştırır ve statik listelerle ulaşılamayan "gömülü" yolları bulur.
- **🧠 Smart Mode (Auto-Everything)**: Ne kelime listesi seçin ne de thread ayarıyla uğraşın. Sistem, hedef sunucunun tepkisine göre vites artırıp azaltır.
- **🎨 Glassmorphism Web UI**: Backend kadar şık, karanlık mod destekli ve gerçek zamanlı (SSE) veri akışlı bir kontrol paneli.

---

## ⚡ Hızlı Başlangıç

### Derleme
```bash
git clone https://github.com/RedRiveRR/ISU-SecOps-Engine
cd ISU-SecOps-Engine
cargo build --release
```

### 🌐 Web Arayüzünü Başlat
```bash
# Otomatik olarak tarayıcı panelini açar
cargo run --release -- web --port 8080
```

### 💻 CLI ile "Hayalet" Tarama
```bash
cargo run --release -- pentest dirbrute "http://target.com" --stealth --proxy "http://p1:8080,socks5://p2:1080"
```

---

## 🛠️ Teknik Yetenekler

| Özellik | Açıklama |
| :--- | :--- |
| **Fingerprinting** | Hedefin WP, Laravel, Docker veya API olduğunu anında teşhis eder. |
| **Heuristic 404** | "Sahte 200" dönen sistemlerde hatayı baseline analiziyle eler. |
| **Adaptive Threading** | Sunucu yorulunca yavaşlar, rahatlayınca ışık hızına çıkar. |
| **Recursive Discovery** | `/admin` bulunca altını da (`/admin/config`) otomatik eşeler. |
| **JSON/CSV Export** | Bulguları temiz raporlar halinde dışa aktarır. |

---

## 🏗️ Geliştirici Ekosistemi

Proje, profesyonel bir yazılım hattı (pipeline) üzerine kuruludur:

- **Commander**: `Justfile` ve `Makefile` ile otomatize edilmiş CI/CD komutları.
- **Linter**: `Clippy` ile katı kod kalitesi denetimi.
- **Formatter**: `rustfmt` ve `taplo` ile nizami kod ve konfigürasyon yapısı.
- **Security**: `cargo-deny` ile tedarik zinciri ve zafiyet analizi.

---

## 📜 Lisans & Yazar

Bu proje [MIT Lisansı](LICENSE) ile korunmaktadır. 

**Geliştiren:** [Mert Kızılırmak (RedRiveRR)](https://github.com/RedRiveRR)

---
<div align="center">
  <sub>Built with ❤️ and 🦀 by the ISU SecOps Team</sub>
</div>

# Test Prosedürü (Testing)

Directory Bruteforcer hem Komut Satırı Arayüzü (CLI) hem de Web Arayüzü modlarında test edilebilir.

## CLI Modu Testi

Merkezi tarama motorunu doğrulamak için:
1. Hedef web sunucusunun çalıştığından emin olun.
2. `pentest dirbrute` alt komutunu çalıştırın.
3. Sonuçların, renkli durum kodlarıyla birlikte hiyerarşik (ağaç) yapıda doğru yazılıp yazılmadığını kontrol edin.

**Çalıştırma:**
```bash
cargo run --release -- pentest dirbrute "http://localhost:8000" --wordlist common.txt
```

### Doğrulama Listesi:
- [ ] Tarama başarıyla başlar.
- [ ] Kelime listesi (wordlist) doğru yüklenir.
- [ ] Bulunan yollar ağaç yapısında görüntülenir.
- [ ] Durum kodları uygun renklerde vurgulanır (200 için Yeşil, 301+ için Sarı, 400+ için Kırmızı).
- [ ] `--stealth` bayrağı ile decoy istekler (`[🥷 STEALTH]`) terminale basılır.
- [ ] `--proxy` parametresi ile istekler belirtilen vekil sunucu üzerinden akar.

## Web UI Modu Testi

Entegre web sunucusunu doğrulamak için:
1. `web` alt komutunu çalıştırın.
2. Tarayıcınızda `http://127.0.0.1:8080` adresine gidin.
3. Test yapılandırmasını girin, "Deep Stealth Mode" seçeneğini aktif edin ve "Launch Scan" butonuna tıklayın.

**Çalıştırma:**
```bash
cargo run --release -- web --port 8080
```

### Doğrulama Listesi:
- [ ] Sunucu başlar ve belirtilen portu dinler.
- [ ] Web arayüzü erişilebilirdir ve doğru şekilde render edilir (Karanlık mod, animasyonlar).
- [ ] "Deep Stealth Mode" aktifken Live Logs sekmesinde mor renkli hayalet logları görünür.
- [ ] "Proxy Engine" alanına girilen virgüllü listeler (rootation) hata vermeden işlenir.
- [ ] Sonuçlar bulundukça gerçek zamanlı (SSE) olarak görüntülenir.
- [ ] İlerleme göstergesi "X yollar taranıyor..." mesajını ve bitişte "Tamamlandı" ibaresini gösterir.

## Birim ve Entegrasyon Testleri

Resmi testler için Rust'ın yerleşik test aracını kullanın:
```bash
cargo test
```
*(Şu an için proje otomatik testler eklenene kadar manuel CLI/Web doğrulaması üzerine kuruludur.)*

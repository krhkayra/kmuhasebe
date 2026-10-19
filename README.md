<div align="center">

# kmuhasebe

**Aylık gelir-gider takibi için sade bir masaüstü uygulaması.**

</div>

---

`kmuhasebe`, aylık gelir ve giderlerin takip edilmesini amaçlayan bir masaüstü ev muhasebe uygulamasıdır.

## Gereksinimler

Derleme için aşağıdaki bağımlılıklar gereklidir:

- GCC / G++
- Qt 6
- Qt Declarative
- KDE Frameworks 6 — Kirigami
- QQC2 Desktop Style
- Breeze Icon Theme
- GNU Make

### Fedora

Gerekli paketleri yüklemek için:

```bash
sudo dnf install gcc-c++ qt6-qtbase-devel qt6-qtdeclarative-devel \
    kf6-kirigami kf6-qqc2-desktop-style breeze-icon-theme
```

## Kurulum ve Derleme

### 1. Depoyu klonla

```bash
git clone https://github.com/kayrakrh/kmuhasebe.git
cd kmuhasebe
```

### 2. Uygulamayı derle

```bash
sudo make
```

### 3. Uygulamayı çalıştır

```bash
./target/release/accounting
```

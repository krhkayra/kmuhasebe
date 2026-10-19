# kmuhasebe
Aylık gelir-gider takibi

### İndirme
```bash
sudo dnf install gcc-c++ qt6-qtbase-devel qt6-qtdeclarative-devel \
         kf6-kirigami kf6-qqc2-desktop-style breeze-icon-theme
```
### Derleme
```bash
git clone https://github.com/kayrakrh/kmuhasebe
cd kmuhasebe
sudo make
./target/release/accounting
```

pkgname="calc"
pkgver="1.0.0"
arch=('x86_64')
pkgrel="1"
build() {
    cargo build --release
}

package() {
    install -Dm755 "../target/release/gtk4-tries" "$pkgdir/usr/bin/$pkgname"
}

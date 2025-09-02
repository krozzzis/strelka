# Strelka

## Сборка и запуск

### Обычная сборка (debug)
```bash
cargo build
cargo run
```

### Release сборка (средние оптимизации)
```bash
cargo build --release
cargo run --release
```

### Production сборка (полные оптимизации)
```bash
cargo build --profile production
cargo run --profile production
```

## Профили сборки

- **debug** (по умолчанию) - для разработки, с отладочной информацией
- **release** - средние оптимизации (opt-level=2), без LTO
- **production** - полные оптимизации (opt-level=3), с LTO и strip

## Логирование

Настройка уровней логирования:
```bash
RUST_LOG=strelka=debug,strelka_core=info,strelka_gui=debug cargo run
```

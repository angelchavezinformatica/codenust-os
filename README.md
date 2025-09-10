# Codenust OS 🦀

**Codenust** es un sistema operativo innovador escrito en **Rust**, inspirado en la filosofía minimalista de **Arch Linux**, pero diseñado con un enfoque moderno en **seguridad, modularidad y personalización**.

---

## 🚀 Filosofía

- **Minimalismo** → Instala solo lo esencial, sin bloatware.
- **Control total** → El usuario decide qué paquetes, servicios y entorno gráfico usar.
- **Seguridad por diseño** → Rust previene errores de memoria comunes en C.
- **Rolling Release** → Actualizaciones continuas, siempre en la última versión estable.

---

## 🧩 Características principales

- **Kernel modular en Rust**: arquitectura tipo _microkernel_ para mayor seguridad y estabilidad.
- **Gestor de procesos y memoria seguro**: evita _buffer overflows_ y _race conditions_.
- **Gestor de paquetes híbrido**: inspirado en `pacman`, con soporte para paquetes tradicionales y contenedores aislados.
- **Instalación mínima**: arranca en modo CLI, con opción de añadir GUI modular (ej. _tiling window manager_).
- **Optimización energética**: ajusta dinámicamente CPU, RAM y GPU en laptops y servidores.

---

## 🏗️ Esquema general

```

            ┌─────────────────────────────┐
            │        Usuario final         │
            │   (CLI / GUI opcional)       │
            └───────────────┬─────────────┘
                            │
             ┌──────────────┴──────────────┐
             │   Servicios del sistema     │
             │   - Shell interactiva       │
             │   - Gestor de paquetes      │
             │   - Control de usuarios     │
             │   - Servicios de red        │
             └──────────────┬──────────────┘
                            │
             ┌──────────────┴──────────────┐
             │     Kernel Codenust (Rust)  │
             │   - Gestión de procesos     │
             │   - Gestión de memoria      │
             │   - Controladores (drivers) │
             │   - Seguridad y permisos    │
             │   - IPC y multitarea        │
             └──────────────┬──────────────┘
                            │
             ┌──────────────┴──────────────┐
             │        Hardware físico       │
             │ CPU │ RAM │ GPU │ Almacen.   │
             │ Red │ Periféricos │ Dispos.  │
             └─────────────────────────────┘

```

---

## 🔥 Innovaciones frente a Arch Linux

1. **Rust en el núcleo** → mayor seguridad en memoria.
1. **Microkernel modular** → componentes intercambiables y personalizables.
1. **Optimización energética inteligente** → pensado para laptops y servidores.

---

## 🤝 Contribución

Cualquier idea, sugerencia o propuesta es bienvenida.

- Reporta problemas en **Issues**.
- Envía parches o propuestas vía **Pull Requests**.

---

## 📝 Licencia

Codenust OS se publicará bajo licencia **MIT**, fomentando el uso libre y educativo.

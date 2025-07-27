# BlockHost 🛡️

Una aplicación de escritorio desarrollada en Rust con GTK4 que permite bloquear dominios no deseados modificando el archivo hosts del sistema. La aplicación incluye múltiples listas de bloqueo y un agente LLM integrado para asistencia.

## 🚀 Características

- **Bloqueo de dominios personalizable**: Selecciona qué categorías de dominios bloquear
- **Múltiples listas de bloqueo**:
  - Facebook y redes sociales
  - Dominios de Steven Black
  - Multi PRO - Protección extendida
  - WindowSpia - Bloquea telemetría de Windows
  - Pornografía - Contenido +18
- **Agente LLM integrado**: Chat con modelo Ollama para asistencia
- **Interfaz gráfica moderna**: Construida con GTK4
- **Limpieza automática de DNS**: Flush automático después de aplicar cambios
- **Sidebar navegable**: Interfaz organizada por secciones

## 🛠️ Tecnologías Utilizadas

- **Rust** - Lenguaje de programación principal
- **GTK4** - Framework de interfaz gráfica
- **Tokio** - Runtime asíncrono
- **Reqwest** - Cliente HTTP para descargas
- **Serde** - Serialización/deserialización JSON
- **Ollama** - Integración con modelo LLM local

## 📋 Requisitos Previos

### Windows
- **Rust**: Instalado desde [rustup.rs](https://rustup.rs/)
- **GTK4**: Descargar e instalar desde [GTK para Windows](https://www.gtk.org/docs/installations/windows/)
- **MSYS2** (recomendado): Para herramientas de desarrollo adicionales
- **Ollama** (opcional): Para el agente LLM - [Descargar Ollama](https://ollama.ai/)

### Linux (Debian/Ubuntu)
```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Instalar dependencias de GTK4
sudo apt update
sudo apt install libgtk-4-dev build-essential pkg-config

# Instalar Ollama (opcional)
curl -fsSL https://ollama.ai/install.sh | sh
```

### Configuración de GTK4 en Windows

1. **Descargar GTK4**:
   - Ir a https://www.gtk.org/docs/installations/windows/
   - Descargar el bundle completo de GTK4

2. **Configurar variables de entorno**:
   ```cmd
   set GTK_BASEPATH=C:\gtk
   set PATH=%GTK_BASEPATH%\bin;%PATH%
   set PKG_CONFIG_PATH=%GTK_BASEPATH%\lib\pkgconfig
   ```

3. **Verificar instalación**:
   ```cmd
   pkg-config --modversion gtk4
   ```

## 🔧 Instalación

1. **Clonar el repositorio**:
   ```bash
   git clone https://github.com/tu-usuario/blockhost.git
   cd blockhost
   ```

2. **Compilar el proyecto**:
   ```bash
   cargo build --release
   ```

3. **Ejecutar la aplicación**:
   ```bash
   cargo run --release
   ```

## 🖥️ Uso

### Bloqueo de Dominios

1. **Ejecutar como Administrador**: 
   - En Windows: Click derecho → "Ejecutar como administrador"
   - En Linux: `sudo ./target/release/my-gtk-app`

2. **Seleccionar categorías**: En la pestaña "Dominios", activa los switches para las categorías que deseas bloquear

3. **Aplicar cambios**: Click en "Apply" para descargar las listas y modificar el archivo hosts

4. **Limpiar bloqueos**: Click en "Limpiar" para restaurar el archivo hosts original

### Agente LLM

1. **Configurar Ollama**:
   ```bash
   # Descargar el modelo (ejemplo)
   ollama pull qwen2:0.6b
   
   # Iniciar el servicio
   ollama serve
   ```

2. **Usar el chat**: Ve a la pestaña "Agente" y escribe tu consulta en el campo de texto

## 📁 Estructura del Proyecto

```
blockhost/
├── src/
│   └── main.rs          # Código principal de la aplicación
├── ui/
│   └── app.ui           # Interfaz de usuario GTK4
├── Cargo.toml           # Configuración de dependencias
├── Cargo.lock           # Lockfile de dependencias
└── README.md            # Este archivo
```

## ⚠️ Consideraciones de Seguridad

- **Permisos de administrador**: La aplicación requiere permisos elevados para modificar el archivo hosts
- **Respaldo**: Se recomienda hacer respaldo del archivo hosts antes de usar la aplicación
- **Antivirus**: Algunos antivirus pueden alertar sobre la modificación del archivo hosts

## 🔧 Configuración Avanzada

### Personalizar Modelo LLM

Edita la variable `model` en `src/main.rs` línea 287:
```rust
let request = OllamaRequest {
    model: "tu-modelo-preferido",  // Cambiar aquí
    prompt: &input,
    stream: true,
};
```

### Agregar Nuevas Listas de Bloqueo

1. Añade un nuevo switch en `ui/app.ui`
2. Agrega la lógica correspondiente en `src/main.rs`
3. Incluye la URL de la nueva lista en la función `download_and_append_file`

## 🐛 Solución de Problemas

### Error: "No se encontró GTK4"
- Verifica que GTK4 esté instalado correctamente
- Configura las variables de entorno necesarias

### Error: "Permiso denegado al modificar hosts"
- Ejecuta la aplicación como administrador/root
- Verifica que el archivo hosts no esté bloqueado por antivirus

### Error: "No se puede conectar con Ollama"
- Verifica que Ollama esté ejecutándose: `ollama serve`
- Confirma que el modelo está descargado: `ollama list`

## 🤝 Contribuciones

Las contribuciones son bienvenidas. Por favor:

1. Fork el repositorio
2. Crea una rama para tu feature: `git checkout -b feature/nueva-funcionalidad`
3. Commit tus cambios: `git commit -am 'Agregar nueva funcionalidad'`
4. Push a la rama: `git push origin feature/nueva-funcionalidad`
5. Crea un Pull Request

## 📄 Licencia

Este proyecto está bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para más detalles.

## 🙏 Agradecimientos

- [StevenBlack/hosts](https://github.com/StevenBlack/hosts) - Por las listas de bloqueo
- [crazy-max/WindowsSpyBlocker](https://github.com/crazy-max/WindowsSpyBlocker) - Por la lista de telemetría de Windows
- [anudeepND/blacklist](https://github.com/anudeepND/blacklist) - Por las listas específicas
- [hagezi/dns-blocklists](https://github.com/hagezi/dns-blocklists) - Por las listas PRO

## 📞 Soporte

Si tienes problemas o preguntas:
- Abre un [Issue](https://github.com/tu-usuario/blockhost/issues)
- Consulta la [Wiki](https://github.com/tu-usuario/blockhost/wiki)
- Usa el agente LLM integrado para ayuda rápida

---

**⚡ Desarrollado con Rust + GTK4 para máximo rendimiento y seguridad**

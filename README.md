# Desarrollo de una API REST en Rust con Actix Web y Diesel ORM

Estás encargado de construir una API REST que maneje operaciones CRUD para un recurso 'Producto' en un sistema de gestión de inventario. La API debe permitir crear, leer, actualizar y eliminar productos, asegurando la consistencia de los datos y manejando adecuadamente los errores del dominio. Los productos tienen atributos como nombre, precio, stock y categoría. Debes asegurar que los nombres de los productos sean únicos y que el precio no sea negativo. La API debe ser capaz de manejar al menos 1000 solicitudes por segundo con una latencia máxima de 50ms.

## Informacion General

| Campo | Valor |
|-------|-------|
| **Tema** | rust-actix-web |
| **Nivel** | junior-l2 |
| **Tipo** | practical |
| **Tiempo estimado** | 8 horas |

## Fases del Reto

### Fase 0: Configuración del Proyecto

**Objetivo:** Obtener el proyecto base funcional enviando el Código Base a un asistente de IA, que lo analizará, corregirá errores y generará un ZIP listo para usar.

**Tiempo estimado:** 15-30 minutos

**Instrucciones:**

- Asegúrate de tener instalado para ejecutar el proyecto: Un IDE o editor de código.
- Copia todo el contenido del campo **Código Base** de este reto — incluyendo el texto de instrucciones que aparece al inicio.
- Abre un asistente de IA (Claude en claude.ai, ChatGPT o Gemini — se recomienda Claude), pega el contenido copiado en el chat y envíalo.
- El asistente analizará los archivos, corregirá errores y generará un archivo ZIP descargable. Descárgalo y extráelo en la carpeta donde quieras trabajar.
- Verifica que el proyecto arranca sin errores.

**Entregable:** El proyecto compila/arranca sin errores.

<details>
<summary>Pistas de conocimiento</summary>

- Copia el Código Base completo incluyendo el texto de instrucciones al inicio — esas instrucciones le indican al asistente exactamente qué hacer con los archivos.
- Si el asistente no genera el ZIP automáticamente al terminar el análisis, escríbele: "genera el ZIP ahora".
- Si el proyecto tiene errores al arrancar, comparte el mensaje de error con el mismo asistente para que lo corrija.

</details>

### Fase 1: Definición del recurso y validación básica

**Objetivo:** Definir el recurso 'Producto' y establecer las validaciones básicas para sus atributos.

**Tiempo estimado:** 2 horas

**Instrucciones:**

- Define el recurso 'Producto' con sus atributos: nombre, precio, stock y categoría.
- Implementa las validaciones para asegurar que el nombre del producto sea único y que el precio no sea negativo.
- Crea un endpoint para crear un nuevo producto y asegúrate de que las validaciones se apliquen correctamente.

**Entregable:** Endpoint para crear un nuevo producto con validaciones aplicadas.

<details>
<summary>Pistas de conocimiento</summary>

- Considera cómo manejarías los errores de validación en el dominio.
- Piensa en cómo asegurarías la unicidad del nombre del producto.

</details>

### Fase 2: Implementación de endpoints CRUD

**Objetivo:** Implementar los endpoints para leer, actualizar y eliminar productos.

**Tiempo estimado:** 3 horas

**Instrucciones:**

- Implementa los endpoints para leer, actualizar y eliminar productos.
- Asegúrate de que los endpoints manejen adecuadamente los errores del dominio, como productos no encontrados o conflictos de actualización.
- Prueba los endpoints para asegurar que funcionan correctamente y que manejan los errores adecuadamente.

**Entregable:** Endpoints para leer, actualizar y eliminar productos con manejo de errores del dominio.

<details>
<summary>Pistas de conocimiento</summary>

- Considera cómo manejarías los errores de dominio en los endpoints.
- Piensa en cómo asegurarías la consistencia de los datos al actualizar o eliminar productos.

</details>

### Fase 3: Optimización y escalabilidad

**Objetivo:** Optimizar la API para manejar al menos 1000 solicitudes por segundo con una latencia máxima de 50ms.

**Tiempo estimado:** 3 horas

**Instrucciones:**

- Analiza el rendimiento de la API y identifica áreas de mejora.
- Implementa las optimizaciones necesarias para alcanzar el rendimiento requerido.
- Realiza pruebas de carga para asegurar que la API cumple con los requisitos de rendimiento.

**Entregable:** API optimizada para manejar al menos 1000 solicitudes por segundo con una latencia máxima de 50ms.

<details>
<summary>Pistas de conocimiento</summary>

- Considera cómo podrías optimizar las consultas a la base de datos.
- Piensa en cómo podrías utilizar técnicas de caché para mejorar el rendimiento.

</details>

## Dimensiones Evaluadas

- **queEs**: ¿Qué es un recurso en una API REST y cuáles son sus atributos en este caso?
- **paraQueSirve**: ¿Para qué sirve cada endpoint CRUD en el contexto de esta API?
- **comoSeUsa**: ¿Cómo se usa un endpoint para crear un nuevo producto y cuáles son las validaciones que se aplican?
- **erroresComunes**: ¿Cuáles son los errores comunes que pueden ocurrir al interactuar con la API y cómo se manejan?
- **queDecisionesImplica**: ¿Qué decisiones de diseño implica la optimización de la API para manejar un alto volumen de solicitudes?

## Criterios de Evaluacion

- Definición correcta del recurso 'Producto' y sus atributos.
- Implementación de las validaciones básicas para los atributos del producto.
- Implementación de los endpoints CRUD con manejo adecuado de errores del dominio.
- Optimización de la API para manejar al menos 1000 solicitudes por segundo con una latencia máxima de 50ms.

---

*Reto generado automaticamente por Challenge Generator - Pragma*

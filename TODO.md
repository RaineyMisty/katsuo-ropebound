## TODO List

### **Camera**:
- `camera`: Moving camera.

### **Collision**:

- `collision`
  - `mod`: CollisionPlugin.
  - `component`: Collision component that attach to player and platform.
  - `calculation`: Collision select and calculate overlap
- `event`: Write an event to send collision graphic information.
- `physics`
  - `collision`: Perform physical analysis on the incoming collision information.

### **Rope**

- `rope`
  - `spawn`: Spawn rope and send event to request Physics component.
  - `visualization`: Rope visualization.
- `physics`
  - `rope`: Rope physics analysis.

### **Maploading**:

- `mapload`:
  - `mod`: MapLoadingPlugin.
  - `mapdata`: Convert the json file in assets into resources.
  - `background`: Spawn background.
  - `platform`: Send platform spawn event.
  - `coin`: Send coin spawn event.
  - `resoruce`: Save map data as resource.

### **Platform**:

- `platform`: Spawn platform as an entity.
  - `mod`: PlatformPlugin.
  - `spawn`: Platform spawn and send collision component attachment request and physics component attachment request.
- `physics`
  - `platform`: Add moving platform logic here.
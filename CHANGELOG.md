## 0.2

- Changed the way events are handled. It doesn’t create an events thread anymore but instead exposes
  a polling interface. It’ll enhance performance and make mono-thread systems work, like Mac OSX.

### 0.1.5

- Internal fix for Mac OSX.

### 0.1.4

- Updated `luminance` dependencies.

### 0.1.3

- Changed the trait bound from `Fn` to `FnOnce` on `Device::draw`.

### 0.1.2

- Made `glfw::InitError` visible.

## 0.1.1

- `Device::draw` added.

## 0.1

- Initial revision.

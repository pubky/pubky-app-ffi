# Pubky App FFI

Cross-platform FFI bindings for the Pubky App, providing native integration for iOS, Android, and Python platforms. This library enables social networking features, user management, content publishing, and data retrieval through the Pubky protocol.

## Building the Bindings

### All Platforms
```
./build.sh all
```

### Platform-Specific Builds
```
./build.sh ios      # iOS only
./build.sh android  # Android only
./build.sh python   # Python only
```

### Run examples
```
cargo run --bin example
```

### Run Tests:
```
cargo test -- --test-threads=1
```

## iOS Integration

### Setup
1. Add the generated XCFramework to your Xcode project:
   - Copy `bindings/ios/PubkyApp.xcframework` to your project
   - Ensure "Copy items if needed" is checked
   - Add to your target's frameworks
2. Include Swift bindings:
   - Add `bindings/ios/pubkyapp.swift` to your project

### Example Usage
```swift
import PubkyApp

class PubkyManager {
    // Create a new user
    func createUser(secretKey: String, homeserver: String) throws {
        let user = PubkyAppUser(
            name: "John Doe",
            bio: "Hello Pubky!",
            image: nil,
            links: [UserLink(title: "Website", url: "https://example.com")],
            status: "Available"
        )
        
        let result = try createUser(
            secretKey: secretKey,
            user: user,
            homeserver: homeserver
        )
        
        if result[0] == "error" {
            throw PubkyError(result[1])
        }
    }
    
    // Create a post
    func createPost(pubky: String) throws {
        let post = PubkyAppPost(
            content: "Hello, Pubky!",
            kind: .Short,
            parent: nil,
            embed: nil,
            attachments: nil
        )
        
        let result = try createPost(pubky: pubky, post: post)
        if result[0] == "error" {
            throw PubkyError(result[1])
        }
    }
}
```

## Android Integration

### Setup
1. Add JNI libraries:
   - Copy contents of `bindings/android/jniLibs` to your project's `app/src/main/jniLibs`
2. Include Kotlin bindings:
   - Copy `bindings/android/pubkyapp.kt` to your project's source directory

### Example Usage
```kotlin
class PubkyManager {
    init {
        System.loadLibrary("pubkyapp")
    }
    
    suspend fun createUser(secretKey: String, homeserver: String) {
        val user = PubkyAppUser(
            name = "John Doe",
            bio = "Hello Pubky!",
            image = null,
            links = listOf(UserLink("Website", "https://example.com")),
            status = "Available"
        )
        
        val result = createUser(secretKey, user, homeserver)
        if (result[0] == "error") {
            throw PubkyException(result[1])
        }
    }
    
    suspend fun createPost(pubky: String) {
        val post = PubkyAppPost(
            content = "Hello, Pubky!",
            kind = PostKind.Short,
            parent = null,
            embed = null,
            attachments = null
        )
        
        val result = createPost(pubky, post)
        if (result[0] == "error") {
            throw PubkyException(result[1])
        }
    }
}
```

## Python Integration

### Setup
1. Install the package:
```
cd bindings/python
pip install .
```

### Example Usage
```python
from pubkyapp import *

def create_user(secret_key: str, homeserver: str):
    user = PubkyAppUser(
        name="John Doe",
        bio="Hello Pubky!",
        image=None,
        links=[UserLink(title="Website", url="https://example.com")],
        status="Available"
    )
    
    result = create_user(secret_key, user, homeserver)
    if result[0] == "error":
        raise Exception(result[1])
    return result[1]

def create_post(pubky: str):
    post = PubkyAppPost(
        content="Hello, Pubky!",
        kind=PostKind.Short,
        parent=None,
        embed=None,
        attachments=None
    )
    
    result = create_post(pubky, post)
    if result[0] == "error":
        raise Exception(result[1])
    return result[1]
```

## Error Handling
All methods return a `Vec<String>` where:
- The first element ([0]) is either "success" or "error"
- The second element ([1]) contains either the result data or error message

It's recommended to wrap all calls in try-catch blocks and handle errors appropriately in your application.

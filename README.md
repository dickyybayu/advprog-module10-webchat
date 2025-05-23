# Tutorial 10 - Asynchronous Programming - YewChat

## Original Code

![original](/screenshot/originalchat.png)

## Creativity

 In this experiment, I enhanced the web client interface by adding two creative features:

### 1. Dynamic Gradient on Login Page
Each time the login page is opened, a different gradient background is displayed. This is implemented using `js_sys::Math::random()` to randomly select from a list of gradients. The goal is to provide a visually refreshing and dynamic user experience upon each visit.

![loginpage](/screenshot/gradientlogin.png)

### 2. Dark Mode Toggle
The chat interface now includes a dark mode toggle. Users can switch between light and dark themes instantly via a button in the chat header. The UI adjusts using Tailwind CSS classes that dynamically change based on state. This provides a more comfortable experience for users in different lighting conditions.

#### Light Mode
![lightmode](/screenshot/lightmodechat.png)

#### Dark Mode
![darkmode](/screenshot/darkmodechat.png)

## Bonus: Rust WebSocket Server for YewChat

In this bonus task, I replaced the original JavaScript-based WebSocket server from Tutorial 3 with a Rust-based WebSocket server adapted from Tutorial 2. The main challenge was that Tutorial 2’s server only handled plain text messages, while Tutorial 3’s YewChat client communicates using a structured JSON format. To address this, I modified the Rust server to parse incoming JSON messages and to serialize outgoing messages into the same format. I introduced a WebSocketMessage struct that mirrors the client’s message schema, supporting register, message, and users message types. The server tracks active users in a HashMap, broadcasting an updated user list whenever a new user joins, and forwards messages between clients using JSON-formatted strings. Although the data is transmitted as plain text, the internal format is JSON-compatible with the frontend.

This was a successful change because the YewChat frontend now works seamlessly with the modified Rust server. All key features, such as user registration, message sending, and user list updates, are fully functional. The system supports real-time updates and dynamic UI changes. Compared to the original JavaScript server, I prefer using Rust due to its strong type safety, performance, and better maintainability for complex asynchronous logic. Overall, this task demonstrates how backend communication logic can be re-implemented in a different language, provided that the protocol format (in this case, serialized JSON) is respected. I believe this showcases how Rust can effectively replace JavaScript for real-time server applications, especially when reliability and scalability are priorities.
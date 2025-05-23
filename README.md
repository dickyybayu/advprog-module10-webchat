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



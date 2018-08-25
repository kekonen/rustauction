<template>
  <div id="app">
    <img alt="Vue logo" src="./assets/logo.png">
    <Login msg="Welcome to My App, please LogIn"/>
  </div>
</template>

<script>
import LoginPage from './components/LoginPage.vue'

export default {
  name: 'app',
  components: {
    LoginPage
  }
}
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
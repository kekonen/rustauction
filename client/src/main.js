import Vue from 'vue'
import VueRouter from 'vue'
import App from './App.vue'
import Login from './Login.vue'

Vue.config.productionTip = false

const routes = [
  { path: '/login', component: Login },
  { path: '/', component: App }
]

const router = new VueRouter({
  routes 
})

const app = new Vue(
	router
// {
//   render: h => h(App)
// }
).$mount('#app')

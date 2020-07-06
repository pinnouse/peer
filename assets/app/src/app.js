import Vue from 'vue';
import App from './App.vue';

let vm = new Vue({
    el: "#app",
    data: function() {
        return {}
    },
    render: function(h) {
        return h(App, { attrs: {} })
    }
});

window.onload = function () {}
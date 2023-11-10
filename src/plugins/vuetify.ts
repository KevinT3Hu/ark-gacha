import '@mdi/font/css/materialdesignicons.css'
import { createVuetify } from 'vuetify'

export default createVuetify({
    icons:{
        defaultSet: 'mdi'
    },
    theme:{
        defaultTheme: 'light',
        themes:{
            light:{
                dark: false,
                colors:{
                    primary: '#FF5722',
                    secondary: '#FF9800',
                    accent: '#FFC107',
                    error: '#F44336',
                    warning: '#FFEB3B',
                    info: '#03A9F4',
                    success: '#4CAF50',
                }
            }
        }
    }
})
import { ref } from 'vue'

export const tables = ref<Array<{ name: string; columns: string[] }>>([
    { name: 'TOM01', columns: ['ORDER_NO', 'ORDER_THICK'] },
    { name: 'TMMCR01', columns: ['MAT_NO', 'MAT_THICK', 'CUSTOMER_ID'] },
    { name: 'TQMTO02', columns: ['ORDER_NO', 'ST_NO', 'ORDER_THICK'] },

])

export const operators = ref<string[]>(['=', '<>', '>', '<', '>=', '<=', 'LIKE', 'IN', 'NOT IN'])

export default {
    tables,
    operators
}





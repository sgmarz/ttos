#include "platform.h"
#include "uart.h"

int main() {
	uart_t *uart = (uart_t *)UART3_BASE_ADDR;
	return 0;
}

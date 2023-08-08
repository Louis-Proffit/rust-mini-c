	.text
	.globl main
main:
	movq $65, %rdi
	movq $0, %rsi
	movq $0, %r11
	cmpq %rsi, %r11
	sete %sil
	addq %rsi, %rdi
	call putchar
	movq $65, %rdi
	movq $1, %r8
	movq $0, %r11
	cmpq %r8, %r11
	sete %r8b
	addq %r8, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	.data


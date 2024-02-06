	.text
	.globl main
main:
	movq $65, %rdi
	movq $0, %r9
	movq $0, %r11
	cmpq %r9, %r11
	sete %r9b
	addq %r9, %rdi
	call putchar
	movq $65, %rdi
	movq $1, %rax
	movq $0, %r11
	cmpq %rax, %r11
	sete %al
	addq %rax, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	.data


	.text
	.globl main
main:
	movq $0, %r9
	movq %r9, %rcx
	movq $65, %rdi
	movq $0, %r11
	cmpq %r9, %r11
	sete %r9b
	addq %r9, %rdi
	call putchar
	movq $1, %r9
	movq %r9, %rcx
	movq $65, %rdi
	movq $0, %r11
	cmpq %r9, %r11
	sete %r9b
	addq %r9, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	.data


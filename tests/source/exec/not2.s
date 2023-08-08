	.text
	.globl main
main:
	movq $0, %rax
	movq %rax, %rdi
	movq $65, %rdi
	movq $0, %r11
	cmpq %rax, %r11
	sete %al
	addq %rax, %rdi
	call putchar
	movq $1, %rax
	movq %rax, %r9
	movq $65, %rdi
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


	.text
	.globl main
main:
	movq $66, %rdi
	movq $1, %rax
	movq $0, %r8
	subq %rax, %r8
	addq %r8, %rdi
	call putchar
	movq $65, %rdi
	movq $1, %r8
	movq $0, %rsi
	subq %r8, %rsi
	movq $0, %rdx
	subq %rsi, %rdx
	addq %rdx, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	.data


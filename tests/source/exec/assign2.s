	.text
	.globl main
main:
	movq $1, %rcx
	movq %rcx, %r9
	movq $65, %rdi
	addq %rcx, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	.data


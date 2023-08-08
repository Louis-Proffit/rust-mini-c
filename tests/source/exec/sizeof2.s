	.text
	.globl main
main:
	movq $65, %rdi
	movq $8, %rcx
	addq %rcx, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $65, %rdi
	movq $16, %rdx
	addq %rdx, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	.data


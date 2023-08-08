	.text
	.globl main
main:
	movq $66, %rdi
	movq $1, %r9
	movq $0, %rcx
	subq %r9, %rcx
	addq %rcx, %rdi
	call putchar
	movq $65, %rdi
	movq $1, %r9
	movq $0, %rcx
	subq %r9, %rcx
	movq $0, %rdx
	subq %rcx, %rdx
	addq %rdx, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	.data


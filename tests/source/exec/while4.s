	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %rdx
L5021:
	movq -8(%rbp), %rcx
	testq %rcx, %rcx
	jz L5010
	movq $1, %rax
	subq %rax, -8(%rbp)
	movq -8(%rbp), %rax
	movq $65, %rdi
	movq -8(%rbp), %rax
	addq %rax, %rdi
	call putchar
	jmp L5021
L5010:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data


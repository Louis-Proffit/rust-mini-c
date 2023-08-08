	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %r8
L4897:
	movq $1, %rdx
	subq %rdx, -8(%rbp)
	movq -8(%rbp), %rax
	movq $1, %rcx
	addq %rcx, %rax
	testq %rax, %rax
	jz L4885
	movq $65, %rdi
	movq -8(%rbp), %rsi
	addq %rsi, %rdi
	call putchar
	jmp L4897
L4885:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data


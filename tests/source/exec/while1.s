	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %r9
L4905:
	movq $1, %rcx
	subq %rcx, -8(%rbp)
	movq -8(%rbp), %r9
	movq $1, %rax
	addq %rax, %r9
	testq %r9, %r9
	jz L4893
	movq $65, %rdi
	movq -8(%rbp), %rcx
	addq %rcx, %rdi
	call putchar
	jmp L4905
L4893:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data


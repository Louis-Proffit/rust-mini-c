	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %rcx
L4943:
	movq $1, %rdx
	subq %rdx, -8(%rbp)
	movq -8(%rbp), %rdx
	testq %rdx, %rdx
	jz L4933
	movq $65, %rdi
	movq -8(%rbp), %r8
	addq %r8, %rdi
	call putchar
	jmp L4943
L4933:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data


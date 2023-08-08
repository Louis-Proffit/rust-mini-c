	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %rax
L4940:
	movq $1, %rcx
	subq %rcx, -8(%rbp)
	movq -8(%rbp), %r9
	testq %r9, %r9
	jz L4930
	movq $65, %rdi
	movq -8(%rbp), %rdx
	addq %rdx, %rdi
	call putchar
	jmp L4940
L4930:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data


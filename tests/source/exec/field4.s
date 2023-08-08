	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $16, %rdi
	call malloc
	movq %rax, -8(%rbp)
	movq -8(%rbp), %r9
	movq $65, %r8
	movq -8(%rbp), %rcx
	movq %r8, 0(%rcx)
	movq -8(%rbp), %rcx
	movq 0(%rcx), %rdi
	call putchar
	movq $66, %r8
	movq -8(%rbp), %rcx
	movq %r8, 8(%rcx)
	movq -8(%rbp), %rax
	movq 8(%rax), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data


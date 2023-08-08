	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $16, %rdi
	call malloc
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rdi
	movq $65, %rcx
	movq -8(%rbp), %rdi
	movq %rcx, 0(%rdi)
	movq -8(%rbp), %rcx
	movq 0(%rcx), %rdi
	call putchar
	movq $66, %r8
	movq -8(%rbp), %rcx
	movq %r8, 8(%rcx)
	movq -8(%rbp), %rdi
	movq 8(%rdi), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data


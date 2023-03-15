	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $16, %rdi
	call malloc
	movq %rax, -8(%rbp)
	addq $0, %rsp
	movq -8(%rbp), %r8
	movq $65, %r9
	movq -8(%rbp), %rax
	movq %rax, 0(%r9)
	movq -8(%rbp), %r9
	movq 0(%r9), %rdi
	call putchar
	addq $0, %rsp
	movq $66, %r9
	movq -8(%rbp), %r8
	movq %r8, 8(%r9)
	movq -8(%rbp), %rax
	movq 8(%rax), %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data


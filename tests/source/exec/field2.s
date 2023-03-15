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
	movq -8(%rbp), %rdi
	movq $65, %rdx
	movq -8(%rbp), %r8
	movq %r8, 0(%rdx)
	movq -8(%rbp), %rsi
	movq 0(%rsi), %rdi
	call putchar
	addq $0, %rsp
	movq $66, %r8
	movq -8(%rbp), %rax
	movq %rax, 8(%r8)
	movq -8(%rbp), %rdx
	movq 8(%rdx), %rdi
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


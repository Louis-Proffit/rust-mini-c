	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq $16, %rdi
	call malloc
	movq %rax, -16(%rbp)
	addq $0, %rsp
	movq -16(%rbp), %rsi
	movq $24, %rdi
	call malloc
	movq %rax, -8(%rbp)
	addq $0, %rsp
	movq -8(%rbp), %rdi
	movq $16, %rdi
	call malloc
	addq $0, %rsp
	movq -8(%rbp), %r8
	movq %r8, 8(%rax)
	movq $65, %rcx
	movq -8(%rbp), %rax
	movq %rax, 0(%rcx)
	movq $66, %rsi
	movq -8(%rbp), %rdi
	movq %rdi, 16(%rsi)
	movq $120, %rax
	movq -8(%rbp), %rdi
	movq 8(%rdi), %rdi
	movq %rdi, 0(%rax)
	movq $121, %rdx
	movq -8(%rbp), %rdi
	movq 8(%rdi), %r9
	movq %r9, 8(%rdx)
	movq -8(%rbp), %r9
	movq 0(%r9), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rax
	movq 8(%rax), %rsi
	movq 0(%rsi), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rdx
	movq 8(%rdx), %rax
	movq 8(%rax), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rdx
	movq 16(%rdx), %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $88, %r8
	movq -16(%rbp), %rdx
	movq %rdx, 0(%r8)
	movq $89, %rax
	movq -16(%rbp), %rcx
	movq %rcx, 8(%rax)
	movq -8(%rbp), %r9
	movq 0(%r9), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rcx
	movq 8(%rcx), %r8
	movq 0(%r8), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rcx
	movq 8(%rcx), %rdi
	movq 8(%rdi), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rax
	movq 16(%rax), %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq -16(%rbp), %r8
	movq -8(%rbp), %r9
	movq %r9, 8(%r8)
	movq -8(%rbp), %rcx
	movq 0(%rcx), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %r9
	movq 8(%r9), %rcx
	movq 0(%rcx), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rdi
	movq 8(%rdi), %rcx
	movq 8(%rcx), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %r8
	movq 16(%r8), %rdi
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


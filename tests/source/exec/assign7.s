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
	movq -16(%rbp), %rdx
	movq $24, %rdi
	call malloc
	movq %rax, -8(%rbp)
	addq $0, %rsp
	movq -8(%rbp), %rcx
	movq $16, %rdi
	call malloc
	addq $0, %rsp
	movq -8(%rbp), %r9
	movq %r9, 8(%rax)
	movq $65, %rdi
	movq -8(%rbp), %rsi
	movq %rsi, 0(%rdi)
	movq $66, %rcx
	movq -8(%rbp), %rsi
	movq %rsi, 16(%rcx)
	movq $120, %rdi
	movq -8(%rbp), %rsi
	movq 8(%rsi), %rdx
	movq %rdx, 0(%rdi)
	movq $121, %r8
	movq -8(%rbp), %rsi
	movq 8(%rsi), %rdx
	movq %rdx, 8(%r8)
	movq -8(%rbp), %rdi
	movq 0(%rdi), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rsi
	movq 8(%rsi), %r8
	movq 0(%r8), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rdx
	movq 8(%rdx), %r8
	movq 8(%r8), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %r9
	movq 16(%r9), %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $88, %r9
	movq -16(%rbp), %r8
	movq %r8, 0(%r9)
	movq $89, %rcx
	movq -16(%rbp), %r8
	movq %r8, 8(%rcx)
	movq -8(%rbp), %r9
	movq 0(%r9), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rsi
	movq 8(%rsi), %r8
	movq 0(%r8), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %r9
	movq 8(%r9), %rdx
	movq 8(%rdx), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rsi
	movq 16(%rsi), %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq -16(%rbp), %rdx
	movq -8(%rbp), %rax
	movq %rax, 8(%rdx)
	movq -8(%rbp), %rdi
	movq 0(%rdi), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rdi
	movq 8(%rdi), %rax
	movq 0(%rax), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rcx
	movq 8(%rcx), %rcx
	movq 8(%rcx), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rdi
	movq 16(%rdi), %rdi
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


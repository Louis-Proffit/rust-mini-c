	.text
	.globl main
f:
	pushq %rbp
	movq %rsp, %rbp
	addq $-120, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, -24(%rbp)
	movq %rcx, -32(%rbp)
	movq %r8, -40(%rbp)
	movq %r9, -48(%rbp)
	movq 16(%rbp), %r10
	movq %r10, -56(%rbp)
	movq 24(%rbp), %r10
	movq %r10, -64(%rbp)
	movq $9, -104(%rbp)
	movq -104(%rbp), %r9
	movq $10, -96(%rbp)
	movq -96(%rbp), %rax
	movq $11, -80(%rbp)
	movq -80(%rbp), %rax
	movq $12, -72(%rbp)
	movq -72(%rbp), %rcx
	movq $13, -88(%rbp)
	movq -88(%rbp), %r9
	movq $14, -112(%rbp)
	movq -112(%rbp), %r9
	movq $15, -120(%rbp)
	movq -120(%rbp), %r8
	movq $65, %rdi
	movq -8(%rbp), %r9
	movq -16(%rbp), %rsi
	imulq %rsi, %r9
	addq %r9, %rdi
	movq -24(%rbp), %rax
	movq -32(%rbp), %rcx
	cqto
	idivq %rcx
	movq -40(%rbp), %rcx
	cqto
	idivq %rcx
	movq -48(%rbp), %r8
	cqto
	idivq %r8
	subq %rax, %rdi
	movq -56(%rbp), %rax
	addq %rax, %rdi
	movq -64(%rbp), %rsi
	subq %rsi, %rdi
	movq -104(%rbp), %rcx
	movq -96(%rbp), %rax
	cmpq %rax, %rcx
	sete %cl
	addq %rcx, %rdi
	movq -80(%rbp), %rax
	movq -72(%rbp), %r8
	cmpq %r8, %rax
	setl %al
	subq %rax, %rdi
	movq -88(%rbp), %r8
	movq -112(%rbp), %r9
	cmpq %r9, %r8
	setle %r8b
	subq %r8, %rdi
	movq -8(%rbp), %rax
	movq -16(%rbp), %rcx
	cmpq %rcx, %rax
	setg %al
	movq -24(%rbp), %r8
	cmpq %r8, %rax
	setg %al
	addq %rax, %rdi
	call putchar
	movq $65, %rdi
	movq $1, %rax
	movq $0, %rsi
	subq %rax, %rsi
	movq -64(%rbp), %rcx
	imulq %rcx, %rsi
	addq %rsi, %rdi
	movq -8(%rbp), %rsi
	movq -16(%rbp), %rcx
	imulq %rcx, %rsi
	movq -24(%rbp), %rcx
	movq -120(%rbp), %r9
	cmpq %r9, %rcx
	setl %cl
	imulq %rcx, %rsi
	addq %rsi, %rdi
	movq -112(%rbp), %rax
	movq -88(%rbp), %r9
	movq -104(%rbp), %rcx
	addq %rcx, %r9
	cmpq %r9, %rax
	setg %al
	movq -16(%rbp), %r9
	imulq %r9, %rax
	movq -16(%rbp), %rsi
	cqto
	idivq %rsi
	movq -16(%rbp), %rsi
	imulq %rsi, %rax
	movq -16(%rbp), %r9
	cqto
	idivq %r9
	subq %rax, %rdi
	movq -80(%rbp), %r8
	movq $0, %rcx
	subq %r8, %rcx
	addq %rcx, %rdi
	movq -80(%rbp), %rsi
	movq -80(%rbp), %rcx
	cmpq %rcx, %rsi
	setne %sil
	movq $0, %r9
	subq %rsi, %r9
	addq %r9, %rdi
	movq -32(%rbp), %rax
	addq %rax, %rdi
	movq -56(%rbp), %r8
	addq %r8, %rdi
	call putchar
	movq $65, %rdi
	movq -8(%rbp), %rax
	movq -16(%rbp), %rsi
	imulq %rsi, %rax
	movq -8(%rbp), %r8
	movq -16(%rbp), %rcx
	movq $2, %rsi
	cmpq %rsi, %rcx
	setne %cl
	movq $4, %rsi
	imulq %rsi, %rcx
	cmpq %rcx, %r8
	setl %r8b
	imulq %r8, %rax
	movq -24(%rbp), %rcx
	cqto
	idivq %rcx
	movq -32(%rbp), %r8
	imulq %r8, %rax
	movq -40(%rbp), %r8
	cqto
	idivq %r8
	movq -48(%rbp), %rsi
	cqto
	idivq %rsi
	addq %rax, %rdi
	movq -56(%rbp), %rax
	movq -64(%rbp), %rsi
	cqto
	idivq %rsi
	movq -104(%rbp), %rcx
	imulq %rcx, %rax
	addq %rax, %rdi
	movq -96(%rbp), %rax
	movq -80(%rbp), %rsi
	imulq %rsi, %rax
	movq -72(%rbp), %r9
	cqto
	idivq %r9
	addq %rax, %rdi
	movq $1, %rax
	movq -88(%rbp), %r9
	cqto
	idivq %r9
	addq %rax, %rdi
	movq $30, %rax
	movq -112(%rbp), %rsi
	cqto
	idivq %rsi
	addq %rax, %rdi
	movq -56(%rbp), %rax
	movq -112(%rbp), %r9
	movq -112(%rbp), %rsi
	cmpq %rsi, %r9
	sete %r9b
	cqto
	idivq %r9
	addq %rax, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-72, %rsp
	movq $1, %rdi
	movq $2, %rsi
	movq $3, %rdx
	movq $4, %rcx
	movq $5, %r8
	movq $6, %r9
	movq $7, -72(%rbp)
	movq $8, %rax
	pushq %rax
	pushq -72(%rbp)
	call f
	addq $16, %rsp
	movq $1, %rdi
	movq $1, %rsi
	movq $1, %rdx
	movq $1, %rcx
	movq $1, %r8
	movq $1, %r9
	movq $1, -64(%rbp)
	movq $1, %rax
	pushq %rax
	pushq -64(%rbp)
	call f
	addq $16, %rsp
	movq $2, %rdi
	movq $2, %rsi
	movq $2, %rdx
	movq $2, %rcx
	movq $2, %r8
	movq $2, %r9
	movq $2, -56(%rbp)
	movq $2, %rax
	pushq %rax
	pushq -56(%rbp)
	call f
	addq $16, %rsp
	movq $8, %rdi
	movq $1, %rsi
	movq $2, %rdx
	movq $3, %rcx
	movq $4, %r8
	movq $5, %r9
	movq $6, -48(%rbp)
	movq $7, %rax
	pushq %rax
	pushq -48(%rbp)
	call f
	addq $16, %rsp
	movq $7, %rdi
	movq $6, %rsi
	movq $5, %rdx
	movq $4, %rcx
	movq $3, %r8
	movq $2, %r9
	movq $1, -40(%rbp)
	movq $8, %rax
	pushq %rax
	pushq -40(%rbp)
	call f
	addq $16, %rsp
	movq $1, %rdx
	movq $0, %rdi
	subq %rdx, %rdi
	movq $2, %rax
	movq $0, %rsi
	subq %rax, %rsi
	movq $3, %rcx
	movq $0, %rdx
	subq %rcx, %rdx
	movq $4, %rax
	movq $0, %rcx
	subq %rax, %rcx
	movq $5, %r9
	movq $0, %r8
	subq %r9, %r8
	movq $6, %rax
	movq $0, %r9
	subq %rax, %r9
	movq $7, %rax
	movq $0, -32(%rbp)
	subq %rax, -32(%rbp)
	movq $8, -24(%rbp)
	movq $0, %rax
	subq -24(%rbp), %rax
	pushq %rax
	pushq -32(%rbp)
	call f
	addq $16, %rsp
	movq $1, %rdi
	movq $2, %r9
	movq $0, %rsi
	subq %r9, %rsi
	movq $3, %rdx
	movq $4, %r9
	movq $0, %rcx
	subq %r9, %rcx
	movq $5, %r8
	movq $6, %rax
	movq $0, %r9
	subq %rax, %r9
	movq $7, -16(%rbp)
	movq $8, -8(%rbp)
	movq $0, %rax
	subq -8(%rbp), %rax
	pushq %rax
	pushq -16(%rbp)
	call f
	addq $16, %rsp
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data


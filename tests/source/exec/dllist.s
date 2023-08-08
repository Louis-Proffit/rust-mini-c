	.text
	.globl main
afficher:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq -16(%rbp), %r8
	movq -16(%rbp), %r8
	movq 0(%r8), %rdi
	call putchar
	movq -16(%rbp), %r8
	movq 8(%r8), %r11
	movq %r11, -16(%rbp)
	movq -16(%rbp), %rdi
L1211:
	movq -16(%rbp), %rsi
	movq -8(%rbp), %r9
	cmpq %r9, %rsi
	setne %sil
	testq %rsi, %rsi
	jz L1200
	movq -16(%rbp), %r9
	movq 0(%r9), %rdi
	call putchar
	movq -16(%rbp), %rax
	movq 8(%rax), %r11
	movq %r11, -16(%rbp)
	movq -16(%rbp), %r8
	jmp L1211
L1200:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
make:
	pushq %rbp
	movq %rsp, %rbp
	addq $-56, %rsp
	movq %rdi, -8(%rbp)
	movq $24, %rdi
	call malloc
	movq %rax, -56(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -40(%rbp)
	movq %rax, -48(%rbp)
	movq -48(%rbp), %r10
	movq -40(%rbp), %r11
	movq %r11, 0(%r10)
	movq %rax, -16(%rbp)
	movq %rax, -32(%rbp)
	movq -32(%rbp), %r10
	movq -16(%rbp), %r11
	movq %r11, 16(%r10)
	movq %rax, -24(%rbp)
	movq -24(%rbp), %r10
	movq -16(%rbp), %r11
	movq %r11, 8(%r10)
	movq %rbp, %rsp
	popq %rbp
	ret
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $65, %rdi
	call make
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rdx
	movq -8(%rbp), %rdi
	call afficher
	movq -8(%rbp), %rdi
	movq $66, %rsi
	call inserer_apres
	movq -8(%rbp), %rdi
	call afficher
	movq -8(%rbp), %rdi
	movq $67, %rsi
	call inserer_apres
	movq -8(%rbp), %rdi
	call afficher
	movq -8(%rbp), %rcx
	movq 8(%rcx), %rdi
	call supprimer
	movq -8(%rbp), %rdi
	call afficher
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
supprimer:
	pushq %rbp
	movq %rsp, %rbp
	addq $-48, %rsp
	movq %rdi, %rax
	movq %rax, -48(%rbp)
	movq -48(%rbp), %r10
	movq 8(%r10), %r11
	movq %r11, -24(%rbp)
	movq %rax, -40(%rbp)
	movq -40(%rbp), %r10
	movq 16(%r10), %r11
	movq %r11, -32(%rbp)
	movq -32(%rbp), %r10
	movq -24(%rbp), %r11
	movq %r11, 8(%r10)
	movq %rax, -16(%rbp)
	movq -16(%rbp), %r10
	movq 16(%r10), %r11
	movq %r11, -8(%rbp)
	movq 8(%rax), %rax
	movq -8(%rbp), %r11
	movq %r11, 16(%rax)
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
inserer_apres:
	pushq %rbp
	movq %rsp, %rbp
	addq $-88, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, %rdi
	call make
	movq %rax, -88(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -80(%rbp)
	movq -80(%rbp), %r10
	movq 8(%r10), %r11
	movq %r11, -64(%rbp)
	movq %rax, -72(%rbp)
	movq -72(%rbp), %r10
	movq -64(%rbp), %r11
	movq %r11, 8(%r10)
	movq %rax, -48(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -56(%rbp)
	movq -56(%rbp), %r10
	movq -48(%rbp), %r11
	movq %r11, 8(%r10)
	movq %rax, -24(%rbp)
	movq %rax, -40(%rbp)
	movq -40(%rbp), %r10
	movq 8(%r10), %r11
	movq %r11, -32(%rbp)
	movq -32(%rbp), %r10
	movq -24(%rbp), %r11
	movq %r11, 16(%r10)
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq -16(%rbp), %r11
	movq %r11, 16(%rax)
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

